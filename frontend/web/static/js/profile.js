// Profile page functionality - TypeScript for settings and goals management
import { getUserSettings, generateUUID } from "./utils.js";
// Populate form fields with current values
function populateFormFields() {
    const settings = getUserSettings();
    if (settings) {
        // Set timezone
        const timezoneSelect = document.getElementById("timezone");
        if (timezoneSelect) {
            timezoneSelect.value = settings.timezone;
        }
        // Set darkmode checkbox
        const darkmodeCheckbox = document.getElementById("darkmode");
        if (darkmodeCheckbox) {
            darkmodeCheckbox.checked = settings.darkmode;
        }
    }
}
// Show loading state for a button
function showLoading(buttonId, spinnerId, textId, loadingText) {
    const button = document.getElementById(buttonId);
    const spinner = document.getElementById(spinnerId);
    const text = document.getElementById(textId);
    if (button)
        button.disabled = true;
    if (text)
        text.textContent = loadingText;
    if (spinner)
        spinner.classList.remove("hidden");
}
// Hide loading state for a button
function hideLoading(buttonId, spinnerId, textId, defaultText) {
    const button = document.getElementById(buttonId);
    const spinner = document.getElementById(spinnerId);
    const text = document.getElementById(textId);
    if (button)
        button.disabled = false;
    if (text)
        text.textContent = defaultText;
    if (spinner)
        spinner.classList.add("hidden");
}
// Show error or success message
function showMessage(message, isError = false) {
    // Remove existing messages
    const existingMessages = document.querySelectorAll(".message-alert");
    existingMessages.forEach((msg) => msg.remove());
    // Create new message
    const messageDiv = document.createElement("div");
    messageDiv.className = `message-alert mb-6 border px-4 py-3 rounded ${isError
        ? "bg-red-50 border-red-200 text-red-700"
        : "bg-green-50 border-green-200 text-green-700"}`;
    messageDiv.textContent = message;
    // Insert at the top of main content
    const main = document.querySelector("main");
    if (main) {
        main.insertBefore(messageDiv, main.firstChild);
    }
    // Auto-hide after 5 seconds
    setTimeout(() => {
        messageDiv.remove();
    }, 5000);
}
document.addEventListener("DOMContentLoaded", function () {
    // Populate form fields with current values
    populateFormFields();
    // Settings form submission
    const settingsForm = document.getElementById("settingsForm");
    if (settingsForm) {
        settingsForm.addEventListener("submit", async function (e) {
            e.preventDefault();
            showLoading("settings-btn", "settings-spinner", "settings-text", "Saving...");
            const formData = new FormData(settingsForm);
            const timezone = formData.get("timezone");
            const darkmode = formData.has("darkmode");
            const settingsData = {
                timezone,
                darkmode,
            };
            try {
                const response = await fetch("/api/v1/settings", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(settingsData),
                });
                if (response.ok) {
                    showMessage("Settings saved successfully!");
                    // Reload page to refresh current settings display and update cookies
                    setTimeout(() => {
                        window.location.reload();
                    }, 1500);
                }
                else {
                    showMessage("Failed to save settings", true);
                }
            }
            catch (error) {
                console.error("Settings error:", error);
                showMessage("Network error occurred while saving settings", true);
            }
            finally {
                hideLoading("settings-btn", "settings-spinner", "settings-text", "Save Settings");
            }
        });
    }
    // Goals form submission
    const goalsForm = document.getElementById("goalsForm");
    if (goalsForm) {
        goalsForm.addEventListener("submit", async function (e) {
            e.preventDefault();
            showLoading("goals-btn", "goals-spinner", "goals-text", "Saving...");
            const formData = new FormData(goalsForm);
            // Only include non-empty values
            const goalsData = {
                user_id: generateUUID(), // Will be overwritten by backend
                calories_consumed: null,
                calories_burned: null,
                active_time_s: null,
            };
            const calorieGoal = formData.get("calorie_goal");
            if (calorieGoal && calorieGoal.trim() !== "") {
                goalsData.calories_consumed = parseInt(calorieGoal, 10);
            }
            const burnGoal = formData.get("burn_goal");
            if (burnGoal && burnGoal.trim() !== "") {
                goalsData.calories_burned = parseInt(burnGoal, 10);
            }
            const activeMinutesGoal = formData.get("active_minutes_goal");
            if (activeMinutesGoal && activeMinutesGoal.trim() !== "") {
                // Convert minutes to seconds for backend
                goalsData.active_time_s = parseInt(activeMinutesGoal, 10) * 60;
            }
            // Check if at least one goal was provided
            if (goalsData.calories_consumed === null &&
                goalsData.calories_burned === null &&
                goalsData.active_time_s === null) {
                showMessage("Please enter at least one goal value", true);
                hideLoading("goals-btn", "goals-spinner", "goals-text", "Save Goals");
                return;
            }
            try {
                const response = await fetch("/api/v1/goals", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(goalsData),
                });
                if (response.ok) {
                    showMessage("Goals saved successfully!");
                    // Clear the form
                    goalsForm.reset();
                    // Reload page to refresh current goals display
                    setTimeout(() => {
                        window.location.reload();
                    }, 1500);
                }
                else {
                    showMessage("Failed to save goals", true);
                }
            }
            catch (error) {
                console.error("Goals error:", error);
                showMessage("Network error occurred while saving goals", true);
            }
            finally {
                hideLoading("goals-btn", "goals-spinner", "goals-text", "Save Goals");
            }
        });
    }
});
//# sourceMappingURL=profile.js.map