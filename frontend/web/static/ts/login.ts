// Login form handling - minimal TypeScript for form interactions

document.addEventListener("DOMContentLoaded", function (): void {
  const loginForm = document.getElementById(
    "loginForm",
  ) as HTMLFormElement | null;
  const loginBtn = document.getElementById(
    "login-btn",
  ) as HTMLButtonElement | null;
  const loginText = document.getElementById("login-text") as HTMLElement | null;
  const loginSpinner = document.getElementById(
    "login-spinner",
  ) as HTMLElement | null;

  if (loginForm) {
    loginForm.addEventListener("submit", function (_e: Event): void {
      // Show loading state
      if (loginBtn) loginBtn.disabled = true;
      if (loginText) loginText.textContent = "Signing in...";
      if (loginSpinner) loginSpinner.classList.remove("hidden");

      // Let the form submit normally to the server
      // The server will handle the authentication and redirect
    });
  }

  // Clear any error styling on input focus
  if (loginForm) {
    const inputs = loginForm.querySelectorAll("input");
    inputs.forEach((input: Element): void => {
      input.addEventListener("focus", function (this: HTMLInputElement): void {
        this.classList.remove("border-red-300");
        this.classList.add("border-gray-300");
      });
    });
  }
});
