// certified spaghetti code

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{NaiveDate, NaiveDateTime};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration};
use tokio_postgres::{NoTls, Row};

#[derive(Deserialize)]
struct ApiResponse {
    all_time: String,
    data: Vec<RevenueEntry>,
}

#[derive(Deserialize, Serialize)]
struct RevenueEntry {
    time: i64,
    revenue: String,
    creator_revenue: String,
}

#[derive(Serialize)]
struct RevenueData {
    time: NaiveDate,
    revenue: f64,
    creator_revenue: f64,
}

async fn connect_to_db() -> tokio_postgres::Client {
    let (client, connection) = tokio_postgres::connect(
        "host=db user=postgres password=postgres dbname=modrinth",
        NoTls,
    )
    .await
    .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    client
}

pub async fn ensure_table_exists(db: &tokio_postgres::Client) {
    let query = "
        CREATE TABLE IF NOT EXISTS platform_revenue (
            time TIMESTAMP PRIMARY KEY,
            revenue DOUBLE PRECISION NOT NULL,
            creator_revenue DOUBLE PRECISION NOT NULL
        );
    ";

    db.execute(query, &[])
        .await
        .expect("Failed to create table");
}

async fn fetch_data(client: &Client) -> Result<ApiResponse, reqwest::Error> {
    let url = "https://api.modrinth.com/v3/payout/platform_revenue";
    let response = client.get(url).send().await?;
    response.json::<ApiResponse>().await
}

async fn store_data(data: Vec<RevenueEntry>, db: &tokio_postgres::Client) {
    let query = "
        INSERT INTO platform_revenue (time, revenue, creator_revenue)
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING;
    ";

    for entry in data {
        let time = NaiveDateTime::from_timestamp(entry.time, 0);
        let revenue: f64 = entry.revenue.parse().unwrap_or(0.0);
        let creator_revenue: f64 = entry.creator_revenue.parse().unwrap_or(0.0);

        db.execute(query, &[&time, &revenue, &creator_revenue])
            .await
            .unwrap_or_else(|e| {
                eprintln!("Failed to insert data: {}", e);
                0
            });
    }
}

async fn scraper_task(client: Client, db: web::Data<tokio_postgres::Client>) {
    let interval = Duration::from_secs(24 * 60 * 60); // 1 day

    let mut interval_timer = time::interval(interval);
    interval_timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        interval_timer.tick().await;
        println!("Running scraper...");
        match fetch_data(&client).await {
            Ok(api_response) => {
                store_data(api_response.data, db.get_ref()).await;
                println!("Data successfully scraped and stored.");
            }
            Err(err) => {
                eprintln!("Error scraping data: {}", err);
            }
        }
    }
}

async fn fetch_data_from_db(
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
    db: &tokio_postgres::Client,
) -> Vec<RevenueData> {
    let rows: Vec<Row> = db.query(query, params).await.expect("Query failed");

    rows.into_iter()
        .map(|row| RevenueData {
            time: row.get(0),
            revenue: row.get(1),
            creator_revenue: row.get(2),
        })
        .collect()
}

async fn get_monthly_data_last_2_years(db: web::Data<tokio_postgres::Client>) -> impl Responder {
    let query = "
        SELECT DATE_TRUNC('month', time)::date AS time, 
               SUM(revenue) AS revenue, 
               SUM(creator_revenue) AS creator_revenue
        FROM platform_revenue
        WHERE time >= NOW() - INTERVAL '2 years'
        GROUP BY DATE_TRUNC('month', time)::date
        ORDER BY time;
    ";
    let data = fetch_data_from_db(query, &[], db.get_ref()).await;
    HttpResponse::Ok().json(data)
}

async fn get_daily_data_last_month(db: web::Data<tokio_postgres::Client>) -> impl Responder {
    let query = "
        SELECT time::date AS time, revenue, creator_revenue
        FROM platform_revenue
        WHERE time >= NOW() - INTERVAL '1 month'
        ORDER BY time;
    ";
    let data = fetch_data_from_db(query, &[], db.get_ref()).await;
    HttpResponse::Ok().json(data)
}

async fn get_weekly_data_last_quarter(db: web::Data<tokio_postgres::Client>) -> impl Responder {
    let query = "
        SELECT DATE_TRUNC('week', time)::date AS time, 
               SUM(revenue) AS revenue, 
               SUM(creator_revenue) AS creator_revenue
        FROM platform_revenue
        WHERE time >= NOW() - INTERVAL '3 months'
        GROUP BY DATE_TRUNC('week', time)::date
        ORDER BY time;
    ";
    let data = fetch_data_from_db(query, &[], db.get_ref()).await;
    HttpResponse::Ok().json(data)
}

async fn get_weekly_data_last_year(db: web::Data<tokio_postgres::Client>) -> impl Responder {
    let query = "
        SELECT DATE_TRUNC('week', time)::date AS time, 
               SUM(revenue) AS revenue, 
               SUM(creator_revenue) AS creator_revenue
        FROM platform_revenue
        WHERE time >= NOW() - INTERVAL '1 year'
        GROUP BY DATE_TRUNC('week', time)::date
        ORDER BY time;
    ";
    let data = fetch_data_from_db(query, &[], db.get_ref()).await;
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let db = web::Data::new(connect_to_db().await);
    ensure_table_exists(db.get_ref()).await;

    tokio::spawn(scraper_task(Client::new(), db.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/2year", web::get().to(get_monthly_data_last_2_years))
            .route("/month", web::get().to(get_daily_data_last_month))
            .route("/quarter", web::get().to(get_weekly_data_last_quarter))
            .route("/year", web::get().to(get_weekly_data_last_year))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
