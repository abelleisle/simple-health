import { api } from "../api.js";
import type { Signin } from "../bindings/Signin.js";

export async function handleLogin(
  username: string,
  password: string,
): Promise<void> {
  try {
    await api.postForm("/login", { username, password });
    window.location.href = "/";
  } catch (error) {
    console.error("Login failed:", error);
    throw new Error("Login failed. Please check your credentials.");
  }
}
