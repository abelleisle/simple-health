import "./style.css";
import type { DailyStats, User } from "./types";
import { api } from "./api";

const user: User = {
  id: "1",
  email: "johnsmith@example.com",
  name: "John Smith",
  calorieGoal: 2121,
};

const today = new Date().toISOString().split("T")[0];
let selectedDate = today;
let backendHealthy = false;
let databaseConnected: boolean | undefined = undefined;
let healthCheckMessage = "Checking...";

const mockStats: DailyStats = {
  date: selectedDate,
  totalCalories: 1245,
  goal: user.calorieGoal,
  entries: [],
  mealBreakdown: {
    breakfast: 320,
    lunch: 485,
    dinner: 440,
    snack: 0,
  },
};

async function checkBackendHealth() {
  const result = await api.healthCheck();
  backendHealthy = result.healthy;
  databaseConnected = result.database;
  healthCheckMessage = result.healthy
    ? "Backend Healthy"
    : `Backend Offline: ${result.message}`;
  renderDashboard();
}

function renderDashboard() {
  const progressPercentage = Math.min(
    (mockStats.totalCalories / mockStats.goal) * 100,
    100,
  );
  const remainingCalories = Math.max(
    mockStats.goal - mockStats.totalCalories,
    0,
  );

  document.querySelector<HTMLDivElement>("#app")!.innerHTML = `
    <div class="min-h-screen bg-gray-50">
      <header class="bg-white shadow">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div class="flex justify-between items-center py-6">
            <div>
              <h1 class="text-3xl font-bold text-gray-900">Simple Health</h1>
              <p class="text-sm text-gray-600">Welcome back, ${user.name}!</p>
            </div>
            <button class="px-4 py-2 text-sm text-red-600 hover:text-red-800 font-medium">
              Sign Out
            </button>
          </div>
        </div>
      </header>

      <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div class="mb-6">
          <label for="date" class="block text-sm font-medium text-gray-700 mb-2">
            Select Date
          </label>
          <input
            type="date"
            id="date"
            value="${selectedDate}"
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        <div class="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          <!-- Calorie Progress Card -->
          <div class="bg-white p-6 rounded-lg shadow col-span-full lg:col-span-2">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">Daily Progress</h2>
            <div class="flex items-center justify-between mb-4">
              <div>
                <p class="text-3xl font-bold text-blue-600">
                  ${mockStats.totalCalories}
                </p>
                <p class="text-sm text-gray-600">of ${mockStats.goal} calories</p>
              </div>
              <div class="text-right">
                <p class="text-lg font-semibold text-gray-900">
                  ${remainingCalories} left
                </p>
                <p class="text-sm text-gray-600">
                  ${progressPercentage.toFixed(1)}% complete
                </p>
              </div>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-3">
              <div
                class="bg-blue-600 h-3 rounded-full transition-all duration-300"
                style="width: ${progressPercentage}%"
              ></div>
            </div>
          </div>

          <!-- Quick Add Card -->
          <div class="bg-white p-6 rounded-lg shadow">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">Quick Add</h2>
            <button class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition-colors">
              Add Food
            </button>
          </div>

          <!-- Meal Breakdown -->
          <div class="bg-white p-6 rounded-lg shadow col-span-full">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">Meal Breakdown</h2>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
              ${Object.entries(mockStats.mealBreakdown)
                .map(
                  ([meal, calories]) => `
                <div class="text-center p-4 bg-gray-50 rounded-lg">
                  <h3 class="font-medium text-gray-900 capitalize">${meal}</h3>
                  <p class="text-2xl font-bold text-blue-600">${calories}</p>
                  <p class="text-sm text-gray-600">calories</p>
                </div>
              `,
                )
                .join("")}
            </div>
          </div>

          <!-- Recent Entries -->
          <div class="bg-white p-6 rounded-lg shadow col-span-full">
            <h2 class="text-lg font-semibold text-gray-900 mb-4">Recent Entries</h2>
            ${
              mockStats.entries.length === 0
                ? `
              <p class="text-gray-600 text-center py-8">
                No food entries for this date. Add some foods to get started!
              </p>
            `
                : `
              <div class="space-y-2">
                ${mockStats.entries
                  .map(
                    (entry) => `
                  <div class="flex justify-between items-center p-3 bg-gray-50 rounded">
                    <div>
                      <p class="font-medium">${entry.name}</p>
                      <p class="text-sm text-gray-600">${entry.type}</p>
                    </div>
                    <p class="font-semibold text-blue-600">${entry.calories} cal</p>
                  </div>
                `,
                  )
                  .join("")}
              </div>
            `
            }
          </div>
        </div>

        <!-- Backend Health Status -->
        <div class="mt-8 pb-4">
          <div class="flex items-center justify-center space-x-6 text-sm">
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 rounded-full ${backendHealthy ? "bg-green-500" : "bg-red-500"}"></div>
              <span class="${backendHealthy ? "text-green-600" : "text-red-600"} font-medium">
                ${healthCheckMessage}
              </span>
            </div>
            ${
              databaseConnected !== undefined
                ? `
            <div class="flex items-center space-x-2">
              <div class="w-3 h-3 rounded-full ${databaseConnected ? "bg-green-500" : "bg-red-500"}"></div>
              <span class="${databaseConnected ? "text-green-600" : "text-red-600"} font-medium">
                Database ${databaseConnected ? "Connected" : "Disconnected"}
              </span>
            </div>
            `
                : ""
            }
          </div>
        </div>
      </main>
    </div>
  `;
}

renderDashboard();

checkBackendHealth();

setInterval(checkBackendHealth, 30000);
