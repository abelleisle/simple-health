// Dashboard functionality - minimal TypeScript for dynamic interactions
import type { Meal } from "./types";

// Generate a random UUID v4
function generateUUID(): string {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0;
    const v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
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
    dateInput.value = now.toISOString().split("T")[0]; // YYYY-MM-DD format
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

// Event listeners
document.addEventListener("DOMContentLoaded", function (): void {
  // Add Food button
  const addFoodBtn = document.getElementById("add-food-btn");
  if (addFoodBtn) {
    addFoodBtn.addEventListener("click", openModal);
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

  // Close modal when clicking outside
  const modal = document.getElementById("food-modal");
  if (modal) {
    modal.addEventListener("click", (e: Event): void => {
      if (e.target === modal) {
        closeModal();
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

        const nameInput = document.getElementById(
          "food-name",
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
          !nameInput.value ||
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
          name: nameInput.value,
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
});
