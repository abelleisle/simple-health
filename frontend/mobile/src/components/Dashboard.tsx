import { useState, useEffect } from 'react';
// import { useAuth } from '../contexts/AuthContext';
import type { DailyStats } from '../types/food';

export function Dashboard() {
  // const { user, logout } = useAuth();

  const user = {
    id: '1',
    email: 'johnsmith@example.com',
    name: 'John Smith',
    calorieGoal: 2121
  }

  const [dailyStats, setDailyStats] = useState<DailyStats | null>(null);
  const [selectedDate, setSelectedDate] = useState(new Date().toISOString().split('T')[0]);

  useEffect(() => {
    // Mock data for now
    const mockStats: DailyStats = {
      date: selectedDate,
      totalCalories: 1245,
      goal: user?.calorieGoal || 2000,
      entries: [],
      mealBreakdown: {
        breakfast: 320,
        lunch: 485,
        dinner: 440,
        snack: 0,
      },
    };
    setDailyStats(mockStats);
  }, [selectedDate, user?.calorieGoal]);

  const progressPercentage = dailyStats 
    ? Math.min((dailyStats.totalCalories / dailyStats.goal) * 100, 100)
    : 0;

  const remainingCalories = dailyStats 
    ? Math.max(dailyStats.goal - dailyStats.totalCalories, 0)
    : 0;

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div>
              <h1 className="text-3xl font-bold text-gray-900">Simple Health</h1>
              <p className="text-sm text-gray-600">Welcome back, {user?.name}!</p>
            </div>
            <button
              // onClick={logout}
              className="px-4 py-2 text-sm text-red-600 hover:text-red-800 font-medium"
            >
              Sign Out
            </button>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-6">
          <label htmlFor="date" className="block text-sm font-medium text-gray-700 mb-2">
            Select Date
          </label>
          <input
            type="date"
            id="date"
            value={selectedDate}
            onChange={(e) => setSelectedDate(e.target.value)}
            className="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>

        {dailyStats && (
          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
            {/* Calorie Progress Card */}
            <div className="bg-white p-6 rounded-lg shadow col-span-full lg:col-span-2">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Daily Progress</h2>
              <div className="flex items-center justify-between mb-4">
                <div>
                  <p className="text-3xl font-bold text-blue-600">
                    {dailyStats.totalCalories}
                  </p>
                  <p className="text-sm text-gray-600">of {dailyStats.goal} calories</p>
                </div>
                <div className="text-right">
                  <p className="text-lg font-semibold text-gray-900">
                    {remainingCalories} left
                  </p>
                  <p className="text-sm text-gray-600">
                    {progressPercentage.toFixed(1)}% complete
                  </p>
                </div>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-3">
                <div
                  className="bg-blue-600 h-3 rounded-full transition-all duration-300"
                  style={{ width: `${progressPercentage}%` }}
                ></div>
              </div>
            </div>

            {/* Quick Add Card */}
            <div className="bg-white p-6 rounded-lg shadow">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Quick Add</h2>
              <button className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 transition-colors">
                Add Food
              </button>
            </div>

            {/* Meal Breakdown */}
            <div className="bg-white p-6 rounded-lg shadow col-span-full">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Meal Breakdown</h2>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                {Object.entries(dailyStats.mealBreakdown).map(([meal, calories]) => (
                  <div key={meal} className="text-center p-4 bg-gray-50 rounded-lg">
                    <h3 className="font-medium text-gray-900 capitalize">{meal}</h3>
                    <p className="text-2xl font-bold text-blue-600">{calories}</p>
                    <p className="text-sm text-gray-600">calories</p>
                  </div>
                ))}
              </div>
            </div>

            {/* Recent Entries */}
            <div className="bg-white p-6 rounded-lg shadow col-span-full">
              <h2 className="text-lg font-semibold text-gray-900 mb-4">Recent Entries</h2>
              {dailyStats.entries.length === 0 ? (
                <p className="text-gray-600 text-center py-8">
                  No food entries for this date. Add some foods to get started!
                </p>
              ) : (
                <div className="space-y-2">
                  {dailyStats.entries.map((entry) => (
                    <div key={entry.id} className="flex justify-between items-center p-3 bg-gray-50 rounded">
                      <div>
                        <p className="font-medium">{entry.food.name}</p>
                        <p className="text-sm text-gray-600">
                          {entry.quantity} {entry.food.unit} • {entry.meal}
                        </p>
                      </div>
                      <p className="font-semibold text-blue-600">{entry.calories} cal</p>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
