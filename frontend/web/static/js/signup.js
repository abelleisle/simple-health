document.addEventListener("DOMContentLoaded", function () {
    const signupForm = document.getElementById("signupForm");
    const signupBtn = document.getElementById("signup-btn");
    const signupText = document.getElementById("signup-text");
    const signupSpinner = document.getElementById("signup-spinner");
    if (signupForm) {
        signupForm.addEventListener("submit", async function (e) {
            e.preventDefault();
            // Show loading state
            if (signupBtn)
                signupBtn.disabled = true;
            if (signupText)
                signupText.textContent = "Creating account...";
            if (signupSpinner)
                signupSpinner.classList.remove("hidden");
            // Get form data
            const formData = new FormData(signupForm);
            const name = formData.get("name");
            const email = formData.get("email");
            const password = formData.get("password");
            const calorieGoal = parseInt(formData.get("calorie_goal"), 10);
            const activeCalories = parseInt(formData.get("active_calories"), 10);
            const activeMinutes = parseInt(formData.get("active_minutes"), 10);
            // Create signup object
            const signupData = {
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
                }
                else {
                    // Handle error response
                    const errorText = await response.text();
                    window.location.href = `/signup?error=${encodeURIComponent(errorText)}`;
                }
            }
            catch (error) {
                console.error("Signup error:", error);
                window.location.href = `/signup?error=${encodeURIComponent("Network error occurred")}`;
            }
            finally {
                // Reset loading state
                if (signupBtn)
                    signupBtn.disabled = false;
                if (signupText)
                    signupText.textContent = "Create Account";
                if (signupSpinner)
                    signupSpinner.classList.add("hidden");
            }
        });
    }
    // Clear any error styling on input focus
    if (signupForm) {
        const inputs = signupForm.querySelectorAll("input");
        inputs.forEach((input) => {
            input.addEventListener("focus", function () {
                this.classList.remove("border-red-300");
                this.classList.add("border-gray-300");
            });
        });
    }
    // Password strength indicator (optional enhancement)
    const passwordInput = document.getElementById("password");
    if (passwordInput) {
        passwordInput.addEventListener("input", function () {
            const password = this.value;
            const isValid = password.length >= 8;
            if (password.length > 0) {
                if (isValid) {
                    this.classList.remove("border-red-300");
                    this.classList.add("border-green-300");
                }
                else {
                    this.classList.remove("border-green-300");
                    this.classList.add("border-red-300");
                }
            }
            else {
                this.classList.remove("border-red-300", "border-green-300");
                this.classList.add("border-gray-300");
            }
        });
    }
});
export {};
//# sourceMappingURL=signup.js.map