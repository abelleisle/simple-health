// Generate a random UUID v4
function generateUUID() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0;
    const v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
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
// Event listeners
document.addEventListener("DOMContentLoaded", function () {
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
    modal.addEventListener("click", (e) => {
      if (e.target === modal) {
        closeModal();
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
      const nameInput = document.getElementById("food-name");
      const caloriesInput = document.getElementById("food-calories");
      const dateInput = document.getElementById("food-date");
      const timeInput = document.getElementById("food-time");
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
      const meal = {
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
    });
  }
});
export {};
//# sourceMappingURL=dashboard.js.map
