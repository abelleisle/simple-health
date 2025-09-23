// Signup form handling - minimal TypeScript for form interactions
import type { Signup } from "./bindings/Signup";

document.addEventListener("DOMContentLoaded", function (): void {
  const signupForm = document.getElementById(
    "signupForm",
  ) as HTMLFormElement | null;
  const signupBtn = document.getElementById(
    "signup-btn",
  ) as HTMLButtonElement | null;
  const signupText = document.getElementById(
    "signup-text",
  ) as HTMLElement | null;
  const signupSpinner = document.getElementById(
    "signup-spinner",
  ) as HTMLElement | null;

  if (signupForm) {
    signupForm.addEventListener(
      "submit",
      async function (e: Event): Promise<void> {
        e.preventDefault();

        // Show loading state
        if (signupBtn) signupBtn.disabled = true;
        if (signupText) signupText.textContent = "Creating account...";
        if (signupSpinner) signupSpinner.classList.remove("hidden");

        // Get form data
        const formData = new FormData(signupForm);
        const name = formData.get("name") as string;
        const email = formData.get("email") as string;
        const password = formData.get("password") as string;
        const calorieGoal = parseInt(
          formData.get("calorie_goal") as string,
          10,
        );
        const activeCalories = parseInt(
          formData.get("active_calories") as string,
          10,
        );
        const activeMinutes = parseInt(
          formData.get("active_minutes") as string,
          10,
        );

        // Create signup object
        const signupData: Signup = {
          name,
          email,
          password,
          goals: {
            user_id: "", // Will be filled by the server
            consumed: calorieGoal || 2000,
            burned: activeCalories || 500,
            active_time_s: activeMinutes ? activeMinutes * 60 : null, // Convert minutes to seconds
          },
          settings: null,
        };

        try {
          const response = await fetch("/api/v1/signup", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(signupData),
          });

          if (response.ok) {
            // Signup successful, redirect to login or dashboard
            window.location.href =
              "/login?success=Account created successfully";
          } else {
            // Handle error response
            const errorText = await response.text();
            window.location.href = `/signup?error=${encodeURIComponent(errorText)}`;
          }
        } catch (error) {
          console.error("Signup error:", error);
          window.location.href = `/signup?error=${encodeURIComponent("Network error occurred")}`;
        } finally {
          // Reset loading state
          if (signupBtn) signupBtn.disabled = false;
          if (signupText) signupText.textContent = "Create Account";
          if (signupSpinner) signupSpinner.classList.add("hidden");
        }
      },
    );
  }

  // Clear any error styling on input focus
  if (signupForm) {
    const inputs = signupForm.querySelectorAll("input");
    inputs.forEach((input: Element): void => {
      input.addEventListener("focus", function (this: HTMLInputElement): void {
        this.classList.remove("border-red-300");
        this.classList.add("border-gray-300");
      });
    });
  }

  // Password strength indicator (optional enhancement)
  const passwordInput = document.getElementById(
    "password",
  ) as HTMLInputElement | null;
  if (passwordInput) {
    passwordInput.addEventListener(
      "input",
      function (this: HTMLInputElement): void {
        const password = this.value;
        const isValid = password.length >= 8;

        if (password.length > 0) {
          if (isValid) {
            this.classList.remove("border-red-300");
            this.classList.add("border-green-300");
          } else {
            this.classList.remove("border-green-300");
            this.classList.add("border-red-300");
          }
        } else {
          this.classList.remove("border-red-300", "border-green-300");
          this.classList.add("border-gray-300");
        }
      },
    );
  }
});
