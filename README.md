# MR Creator Rev

MR Creator Rev is a web application that provides insights through interactive data visualizations. It combines a Vue.js frontend served using Nginx and a Rust backend using Actix-web for high performance using Docker for easy deployment.

## Features

- **Dynamic Charts**: Powered by Chart.js and Vue Chart.js.
- **Real-Time Data**: Backend scraper fetches and stores data periodically.
- **RESTful API**: Endpoints for various data ranges.

## Architecture

The application consists of the following components working together:

- **Scraper**: A background task in the backend that periodically fetches revenue data from the Modrinth API and stores it in the database.
- **Database**: A PostgreSQL database that stores the collected data for efficient retrieval and aggregation.
- **Backend**: A Rust server using Actix-web that serves as an API layer, fetching data from the database and providing it to the frontend.
- **Frontend**: A Vue.js application that consumes the API endpoints to display interactive charts and graphs.

**Workflow:**

1. The **Scraper** runs at scheduled intervals, retrieving the latest revenue data.
2. Retrieved data is saved into the **Database**, maintaining a history for trend analysis.
3. The **Backend** handles client requests, queries the database, and returns aggregated data.
4. The **Frontend** requests data from the backend and renders it using Chart.js for visualization.

## Prerequisites

- **Node.js** (with `pnpm`)
- **Rust** (with `cargo`)

## Installation

### Frontend

1. Navigate to the `frontend` directory:

    ```bash
    cd frontend
    ```

2. Install dependencies:

    ```bash
    pnpm install
    ```

3. Start the development server:

    ```bash
    pnpm dev
    ```

### Backend

1. Navigate to the `backend` directory:

    ```bash
    cd backend
    ```

2. Run the server:

    ```bash
    cargo run
    ```

## API Endpoints

- `/2year` - Monthly data for the last 2 years
- `/year` - Weekly data for the last year
- `/quarter` - Weekly data for the last quarter
- `/month` - Daily data for the last month

## Deployment with Docker

Build and run the services:

```bash
docker-compose up --build
```
