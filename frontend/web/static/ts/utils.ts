// Utility functions for Simple Health frontend

// Utility function to get cookie value
export function getCookie(name: string): string | null {
  const value = `; ${document.cookie}`;
  const parts = value.split(`; ${name}=`);
  if (parts.length === 2) return parts.pop()?.split(";").shift() || null;
  return null;
}

// Parse settings from cookie
export function getUserSettings(): {
  timezone: string;
  darkmode: boolean;
} | null {
  try {
    const settingsCookie = getCookie("settings");
    if (!settingsCookie) return null;

    const settings = JSON.parse(decodeURIComponent(settingsCookie));
    return settings;
  } catch (error) {
    console.error("Failed to parse settings cookie:", error);
    return null;
  }
}

// Get user's timezone with fallback
export function getUserTimezone(): string {
  const settings = getUserSettings();
  return settings?.timezone || "UTC";
}

// Get current date and time in user's timezone
export function getCurrentDateTimeInUserTimezone(): {
  date: string;
  time: string;
} {
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

// Generate a random UUID v4
export function generateUUID(): string {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0;
    const v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}
