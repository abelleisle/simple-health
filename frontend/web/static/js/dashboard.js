// Generate a random UUID v4
function generateUUID() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0;
    const v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}
// Populate meal type dropdown
function populateMealTypeDropdown() {
  // TODO this is trash and shouldn't be hardcoded.
  // Gotta find some way to extract the string literals from the MealType
  // union to fill this array.
  const mealTypes = ["Breakfast", "Lunch", "Dinner", "Snack", "Coffee"];
  const selectElement = document.getElementById("food-type");
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
function populateActivityTypeDropdown() {
  const activityTypes = ["Walk", "Run", "Hike", "Bike", "Ski"];
  const selectElement = document.getElementById("activity-type");
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
function openModal() {
  const modal = document.getElementById("food-modal");
  modal?.classList.remove("hidden");
  // Set current date and time as defaults
  const now = new Date();
  // Set current date
  const dateInput = document.getElementById("food-date");
  if (dateInput) {
    dateInput.value = now.toISOString().split("T")[0]; // YYYY-MM-DD format
  }
  // Set current time
  const timeInput = document.getElementById("food-time");
  if (timeInput) {
    const hours = now.getHours().toString().padStart(2, "0");
    const minutes = now.getMinutes().toString().padStart(2, "0");
    timeInput.value = `${hours}:${minutes}`; // HH:MM format
  }
}
function closeModal() {
  const modal = document.getElementById("food-modal");
  modal?.classList.add("hidden");
  // Reset form
  const form = document.getElementById("food-form");
  form?.reset();
}
function openActivityModal() {
  const modal = document.getElementById("activity-modal");
  modal?.classList.remove("hidden");
  // Set current date and time as defaults
  const now = new Date();
  // Set current date
  const dateInput = document.getElementById("activity-date");
  if (dateInput) {
    dateInput.value = now.toISOString().split("T")[0]; // YYYY-MM-DD format
  }
  // Set current time
  const timeInput = document.getElementById("activity-time");
  if (timeInput) {
    const hours = now.getHours().toString().padStart(2, "0");
    const minutes = now.getMinutes().toString().padStart(2, "0");
    timeInput.value = `${hours}:${minutes}`; // HH:MM format
  }
}
function closeActivityModal() {
  const modal = document.getElementById("activity-modal");
  modal?.classList.add("hidden");
  // Reset form
  const form = document.getElementById("activity-form");
  form?.reset();
}
// Event listeners
document.addEventListener("DOMContentLoaded", function () {
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
    modal.addEventListener("click", (e) => {
      if (e.target === modal) {
        closeModal();
      }
    });
  }
  // Close activity modal when clicking outside
  const activityModal = document.getElementById("activity-modal");
  if (activityModal) {
    activityModal.addEventListener("click", (e) => {
      if (e.target === activityModal) {
        closeActivityModal();
      }
    });
  }
  // Date change handler
  const dateInput = document.getElementById("date");
  if (dateInput) {
    dateInput.addEventListener("change", function () {
      // Redirect to dashboard with new date
      window.location.href = `/?date=${this.value}`;
    });
  }
  // Form submission
  const foodForm = document.getElementById("food-form");
  if (foodForm) {
    foodForm.addEventListener("submit", async function (e) {
      e.preventDefault();
      const typeSelect = document.getElementById("food-type");
      const descriptionInput = document.getElementById("food-description");
      const caloriesInput = document.getElementById("food-calories");
      const dateInput = document.getElementById("food-date");
      const timeInput = document.getElementById("food-time");
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
      const meal = {
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
    });
  }
  // Activity form submission
  const activityForm = document.getElementById("activity-form");
  if (activityForm) {
    activityForm.addEventListener("submit", async function (e) {
      e.preventDefault();
      const typeSelect = document.getElementById("activity-type");
      const descriptionInput = document.getElementById("activity-description");
      const caloriesInput = document.getElementById("activity-calories");
      const durationInput = document.getElementById("activity-duration");
      const dateInput = document.getElementById("activity-date");
      const timeInput = document.getElementById("activity-time");
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
      // Convert duration from HH:MM to Duration format (optional)
      let duration = null;
      if (durationInput.value) {
        const [hours, minutes] = durationInput.value.split(":").map(Number);
        // Convert to total seconds for now - we'll need to adjust based on backend Duration handling
        duration = (hours * 60 + minutes) * 60;
      }
      const activity = {
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
    });
  }
});
export {};
//# sourceMappingURL=dashboard.js.map
