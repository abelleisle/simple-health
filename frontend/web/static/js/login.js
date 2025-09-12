// Login form handling - minimal JavaScript for form interactions

document.addEventListener("DOMContentLoaded", function () {
  const loginForm = document.getElementById("loginForm");
  const loginBtn = document.getElementById("login-btn");
  const loginText = document.getElementById("login-text");
  const loginSpinner = document.getElementById("login-spinner");

  if (loginForm) {
    loginForm.addEventListener("submit", function (e) {
      // Show loading state
      loginBtn.disabled = true;
      loginText.textContent = "Signing in...";
      loginSpinner.classList.remove("hidden");

      // Let the form submit normally to the server
      // The server will handle the authentication and redirect
    });
  }

  // Clear any error styling on input focus
  const inputs = loginForm.querySelectorAll("input");
  inputs.forEach((input) => {
    input.addEventListener("focus", function () {
      this.classList.remove("border-red-300");
      this.classList.add("border-gray-300");
    });
  });
});
