"use strict";
// Dashboard functionality - minimal TypeScript for dynamic interactions
// Modal management
function openModal() {
  const modal = document.getElementById("food-modal");
  modal?.classList.remove("hidden");
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
  // Form submission (if you want to handle via AJAX instead of regular form submission)
  const foodForm = document.getElementById("food-form");
  if (foodForm) {
    foodForm.addEventListener("submit", async function (_e) {
      // Let the form submit normally to the server
      // If you want AJAX handling, uncomment below:
      /*
                e.preventDefault();
                
                const formData = new FormData(this);
                try {
                    const response = await fetch('/add-food', {
                        method: 'POST',
                        body: formData
                    });
                    
                    if (response.ok) {
                        window.location.reload();
                    } else {
                        alert('Failed to add food entry');
                    }
                } catch (error) {
                    alert('Error adding food entry');
                }
                */
    });
  }
});
//# sourceMappingURL=dashboard.js.map
