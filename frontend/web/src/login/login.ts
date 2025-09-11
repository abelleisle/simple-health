import { api } from "../api.js";
import type { Signin } from "../bindings/Signin.js";

export async function handleLogin(
  username: string,
  password: string,
): Promise<void> {
  const signinData: Signin = {
    username,
    password,
  };

  try {
    await api.post("/login", signinData);
    window.location.href = "/";
  } catch (error) {
    console.error("Login failed:", error);
    throw new Error("Login failed. Please check your credentials.");
  }
}
