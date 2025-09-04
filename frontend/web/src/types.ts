export interface DailyStats {
  date: string;
  totalCalories: number;
  goal: number;
  entries: FoodEntry[];
  mealBreakdown: {
    breakfast: number;
    lunch: number;
    dinner: number;
    snack: number;
  };
}

export interface FoodEntry {
  id: string;
  name: string;
  calories: number;
  entry_type: "Meal" | "Snack" | "Drink";
  time: string;
}

export interface User {
  id: string;
  email: string;
  name: string;
  calorieGoal: number;
}
