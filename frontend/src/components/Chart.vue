<template>
    <div class="chart-container">
        <select v-model="selectedTimeframe" @change="fetchData">
            <option value="month">Last Month (Daily)</option>
            <option value="quarter">Last Quarter (Weekly)</option>
            <option value="year">Last Year (Weekly)</option>
            <option value="2year">Last 2 Years (Monthly)</option>
        </select>
        <Line v-if="chartData" :data="chartData" :options="chartOptions" />
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Line } from "vue-chartjs";
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend,
} from "chart.js";

// Register ChartJS components
ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

interface RevenueData {
    time: string;
    revenue: number;
    creator_revenue: number;
}

const selectedTimeframe = ref("month");
const chartData = ref<{
    labels: string[];
    datasets: {
        label: string;
        data: number[];
        borderColor: string;
        tension: number;
    }[];
} | null>(null);

const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    interaction: {
        mode: "index" as "index",
        intersect: false,
    },
    scales: {
        y: {
            beginAtZero: true,
            ticks: {
                callback: function (tickValue: number | string) {
                    if (typeof tickValue === "number") {
                        return `$${tickValue.toFixed(2)}`;
                    }
                    return tickValue;
                },
            },
        },
    },
    plugins: {
        legend: {
            position: "top" as const,
        },
        title: {
            display: true,
            text: "Platform Revenue",
        },
    },
};

async function fetchData() {
    try {
        const response = await fetch(
            `${__API_URL__}${selectedTimeframe.value}`
        );
        const data: RevenueData[] = await response.json();

        chartData.value = {
            labels: data.map((item) =>
                new Date(item.time).toLocaleDateString()
            ),
            datasets: [
                {
                    label: "Total Revenue",
                    data: data.map((item) => item.revenue),
                    borderColor: "rgb(75, 192, 192)",
                    tension: 0.1,
                },
                {
                    label: "Creator Revenue",
                    data: data.map((item) => item.creator_revenue),
                    borderColor: "rgb(255, 99, 132)",
                    tension: 0.1,
                },
            ],
        };
    } catch (error) {
        console.error("Error fetching data:", error);
    }
}

onMounted(() => {
    fetchData();
});
</script>

<style scoped>
.chart-container {
    width: 100%;
    height: 80vh;
}

select {
    margin-bottom: 20px;
    padding: 8px;
    font-size: 16px;
}
</style>
