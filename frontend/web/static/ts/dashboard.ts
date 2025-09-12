// Dashboard functionality - minimal TypeScript for dynamic interactions

// Modal management
function openModal(): void {
  const modal = document.getElementById("food-modal");
  modal?.classList.remove("hidden");
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

  // Form submission (if you want to handle via AJAX instead of regular form submission)
  const foodForm = document.getElementById(
    "food-form",
  ) as HTMLFormElement | null;
  if (foodForm) {
    foodForm.addEventListener(
      "submit",
      async function (_e: Event): Promise<void> {
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
      },
    );
  }
});
