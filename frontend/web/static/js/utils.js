// Utility functions for Simple Health frontend
// Utility function to get cookie value
export function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2)
        return parts.pop()?.split(";").shift() || null;
    return null;
}
// Parse settings from cookie
export function getUserSettings() {
    try {
        const settingsCookie = getCookie("settings");
        if (!settingsCookie)
            return null;
        const settings = JSON.parse(decodeURIComponent(settingsCookie));
        return settings;
    }
    catch (error) {
        console.error("Failed to parse settings cookie:", error);
        return null;
    }
}
// Get user's timezone with fallback
export function getUserTimezone() {
    const settings = getUserSettings();
    return settings?.timezone || "UTC";
}
// Get current date and time in user's timezone
export function getCurrentDateTimeInUserTimezone() {
    const userTimezone = getUserTimezone();
    const now = new Date();
    // Format date as YYYY-MM-DD
    const date = new Intl.DateTimeFormat("en-CA", {
        timeZone: userTimezone,
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
    }).format(now);
    // Format time as HH:MM
    const time = new Intl.DateTimeFormat("en-GB", {
        timeZone: userTimezone,
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
    }).format(now);
    return { date, time };
}
// Convert date/time in user's timezone to UTC ISO string
export function convertUserDateTimeToUTC(date, time) {
    const userTimezone = getUserTimezone();
    // Simple approach: create the date string and let the browser handle conversion
    // Since the user entered this time intending it to be in their timezone,
    // we need to create a Date object that represents that moment in their timezone
    const dateTimeString = `${date}T${time}:00`;
    // Create a "fake" UTC date first
    const fakeUtcDate = new Date(dateTimeString + "Z");
    // Now format this date AS IF it were in the user's timezone to see what UTC time that would be
    const utcTimeInUserTz = new Intl.DateTimeFormat("sv-SE", {
        timeZone: "UTC",
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
    }).format(fakeUtcDate);
    const userTimeInUserTz = new Intl.DateTimeFormat("sv-SE", {
        timeZone: userTimezone,
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
    }).format(fakeUtcDate);
    // Calculate the difference and adjust
    const utcMs = new Date(utcTimeInUserTz.replace(" ", "T") + "Z").getTime();
    const userMs = new Date(userTimeInUserTz.replace(" ", "T") + "Z").getTime();
    const offsetMs = utcMs - userMs;
    // Apply the offset to get the correct UTC time
    const correctUtcDate = new Date(fakeUtcDate.getTime() + offsetMs);
    return correctUtcDate.toISOString();
}
// Generate a random UUID v4
export function generateUUID() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0;
        const v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}
//# sourceMappingURL=utils.js.map