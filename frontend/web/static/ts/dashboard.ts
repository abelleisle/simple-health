// Dashboard functionality - minimal TypeScript for dynamic interactions
import type { Meal, MealType, Activity, ActivityType } from "./types";

// Generate a random UUID v4
function generateUUID(): string {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0;
    const v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

// Populate meal type dropdown
function populateMealTypeDropdown(): void {
  // TODO this is trash and shouldn't be hardcoded.
  // Gotta find some way to extract the string literals from the MealType
  // union to fill this array.
  const mealTypes: MealType[] = [
    "Breakfast",
    "Lunch",
    "Dinner",
    "Snack",
    "Coffee",
  ];
  const selectElement = document.getElementById(
    "food-type",
  ) as HTMLSelectElement;

  if (selectElement) {
    // Clear existing options except the first one
    selectElement.innerHTML = '<option value="">Select meal type</option>';

    // Add options for each meal type
    mealTypes.forEach((type) => {
      const option = document.createElement("option");
      option.value = type;
      option.textContent = type;
      selectElement.appendChild(option);
    });
  }
}

// Populate activity type dropdown
function populateActivityTypeDropdown(): void {
  const activityTypes: ActivityType[] = ["Walk", "Run", "Hike", "Bike", "Ski"];
  const selectElement = document.getElementById(
    "activity-type",
  ) as HTMLSelectElement;

  if (selectElement) {
    selectElement.innerHTML = '<option value="">Select activity type</option>';

    activityTypes.forEach((type) => {
      const option = document.createElement("option");
      option.value = type;
      option.textContent = type;
      selectElement.appendChild(option);
    });
  }
}

// Modal management
function openModal(): void {
  const modal = document.getElementById("food-modal");
  modal?.classList.remove("hidden");

  // Set current date and time as defaults
  const now = new Date();

  // Set current date
  const dateInput = document.getElementById(
    "food-date",
  ) as HTMLInputElement | null;
  if (dateInput) {
    const year = now.getFullYear();
    const month = (now.getMonth() + 1).toString().padStart(2, "0");
    const day = now.getDate().toString().padStart(2, "0");
    dateInput.value = `${year}-${month}-${day}`; // YYYY-MM-DD format in local time
  }

  // Set current time
  const timeInput = document.getElementById(
    "food-time",
  ) as HTMLInputElement | null;
  if (timeInput) {
    const hours = now.getHours().toString().padStart(2, "0");
    const minutes = now.getMinutes().toString().padStart(2, "0");
    timeInput.value = `${hours}:${minutes}`; // HH:MM format
  }
}

function closeModal(): void {
  const modal = document.getElementById("food-modal");
  modal?.classList.add("hidden");

  // Reset form
  const form = document.getElementById("food-form") as HTMLFormElement | null;
  form?.reset();
}

function openActivityModal(): void {
  const modal = document.getElementById("activity-modal");
  modal?.classList.remove("hidden");

  // Set current date and time as defaults
  const now = new Date();

  // Set current date
  const dateInput = document.getElementById(
    "activity-date",
  ) as HTMLInputElement | null;
  if (dateInput) {
    const year = now.getFullYear();
    const month = (now.getMonth() + 1).toString().padStart(2, "0");
    const day = now.getDate().toString().padStart(2, "0");
    dateInput.value = `${year}-${month}-${day}`; // YYYY-MM-DD format in local time
  }

  // Set current time
  const timeInput = document.getElementById(
    "activity-time",
  ) as HTMLInputElement | null;
  if (timeInput) {
    const hours = now.getHours().toString().padStart(2, "0");
    const minutes = now.getMinutes().toString().padStart(2, "0");
    timeInput.value = `${hours}:${minutes}`; // HH:MM format
  }
}

function closeActivityModal(): void {
  const modal = document.getElementById("activity-modal");
  modal?.classList.add("hidden");

  // Reset form
  const form = document.getElementById(
    "activity-form",
  ) as HTMLFormElement | null;
  form?.reset();
}

// Event listeners
document.addEventListener("DOMContentLoaded", function (): void {
  // Populate dropdowns
  populateMealTypeDropdown();
  populateActivityTypeDropdown();

  // Add Food button
  const addFoodBtn = document.getElementById("add-food-btn");
  if (addFoodBtn) {
    addFoodBtn.addEventListener("click", openModal);
  }

  // Add Activity button
  const addActivityBtn = document.getElementById("add-activity-btn");
  if (addActivityBtn) {
    addActivityBtn.addEventListener("click", openActivityModal);
  }

  // Close modal button
  const closeModalBtn = document.getElementById("close-modal-btn");
  if (closeModalBtn) {
    closeModalBtn.addEventListener("click", closeModal);
  }

  // Cancel button
  const cancelBtn = document.getElementById("cancel-btn");
  if (cancelBtn) {
    cancelBtn.addEventListener("click", closeModal);
  }

  // Close activity modal button
  const closeActivityModalBtn = document.getElementById(
    "close-activity-modal-btn",
  );
  if (closeActivityModalBtn) {
    closeActivityModalBtn.addEventListener("click", closeActivityModal);
  }

  // Cancel activity button
  const cancelActivityBtn = document.getElementById("cancel-activity-btn");
  if (cancelActivityBtn) {
    cancelActivityBtn.addEventListener("click", closeActivityModal);
  }

  // Close modal when clicking outside
  const modal = document.getElementById("food-modal");
  if (modal) {
    modal.addEventListener("click", (e: Event): void => {
      if (e.target === modal) {
        closeModal();
      }
    });
  }

  // Close activity modal when clicking outside
  const activityModal = document.getElementById("activity-modal");
  if (activityModal) {
    activityModal.addEventListener("click", (e: Event): void => {
      if (e.target === activityModal) {
        closeActivityModal();
      }
    });
  }

  // Date change handler
  const dateInput = document.getElementById("date") as HTMLInputElement | null;
  if (dateInput) {
    dateInput.addEventListener(
      "change",
      function (this: HTMLInputElement): void {
        // Redirect to dashboard with new date
        window.location.href = `/?date=${this.value}`;
      },
    );
  }

  // Form submission
  const foodForm = document.getElementById(
    "food-form",
  ) as HTMLFormElement | null;
  if (foodForm) {
    foodForm.addEventListener(
      "submit",
      async function (e: Event): Promise<void> {
        e.preventDefault();

        const typeSelect = document.getElementById(
          "food-type",
        ) as HTMLSelectElement;
        const descriptionInput = document.getElementById(
          "food-description",
        ) as HTMLInputElement;
        const caloriesInput = document.getElementById(
          "food-calories",
        ) as HTMLInputElement;
        const dateInput = document.getElementById(
          "food-date",
        ) as HTMLInputElement;
        const timeInput = document.getElementById(
          "food-time",
        ) as HTMLInputElement;

        if (
          !typeSelect.value ||
          !descriptionInput.value ||
          !caloriesInput.value ||
          !dateInput.value ||
          !timeInput.value
        ) {
          alert("Please fill in all required fields");
          return;
        }

        // Combine date and time into ISO string
        const created_at = new Date(
          `${dateInput.value}T${timeInput.value}`,
        ).toISOString();

        const meal: Meal = {
          id: generateUUID(),
          user_id: generateUUID(), // You might want to get this from user session instead
          name: typeSelect.value,
          description: descriptionInput.value,
          calories: parseInt(caloriesInput.value, 10),
          created_at: created_at,
        };

        try {
          const response = await fetch("/api/v1/meal", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(meal),
          });

          if (response.ok) {
            closeModal();
            window.location.reload();
          } else {
            alert("Failed to add meal");
          }
        } catch (error) {
          console.error("Error adding meal:", error);
          alert("Error adding meal");
        }
      },
    );
  }

  // Activity form submission
  const activityForm = document.getElementById(
    "activity-form",
  ) as HTMLFormElement | null;
  if (activityForm) {
    activityForm.addEventListener(
      "submit",
      async function (e: Event): Promise<void> {
        e.preventDefault();

        const typeSelect = document.getElementById(
          "activity-type",
        ) as HTMLSelectElement;
        const descriptionInput = document.getElementById(
          "activity-description",
        ) as HTMLInputElement;
        const caloriesInput = document.getElementById(
          "activity-calories",
        ) as HTMLInputElement;
        const durationInput = document.getElementById(
          "activity-duration",
        ) as HTMLInputElement;
        const dateInput = document.getElementById(
          "activity-date",
        ) as HTMLInputElement;
        const timeInput = document.getElementById(
          "activity-time",
        ) as HTMLInputElement;

        if (
          !typeSelect.value ||
          !descriptionInput.value ||
          !caloriesInput.value ||
          !dateInput.value ||
          !timeInput.value
        ) {
          alert("Please fill in all required fields");
          return;
        }

        // Combine date and time into ISO string
        const created_at = new Date(
          `${dateInput.value}T${timeInput.value}`,
        ).toISOString();

        // Get duration directly as seconds (optional)
        let duration = null;
        if (durationInput.value) {
          duration = parseInt(durationInput.value, 10) * 60;
        }

        const activity: Activity = {
          id: generateUUID(),
          user_id: generateUUID(), // You might want to get this from user session instead
          name: typeSelect.value,
          description: descriptionInput.value,
          calories: parseInt(caloriesInput.value, 10),
          duration_s: duration,
          created_at: created_at,
        };

        try {
          const response = await fetch("/api/v1/activity", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(activity),
          });

          if (response.ok) {
            closeActivityModal();
            window.location.reload();
          } else {
            alert("Failed to add activity");
          }
        } catch (error) {
          console.error("Error adding activity:", error);
          alert("Error adding activity");
        }
      },
    );
  }
});
