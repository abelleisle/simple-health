class ApiManager {
  private baseUrl: string;

  constructor() {
    this.baseUrl = this.getBaseUrl();
  }

  private getBaseUrl(): string {
    if (import.meta.env.DEV) {
      return "http://localhost:3000/api/v1";
    }
    return "/api/v1";
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {},
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;

    const response = await fetch(url, {
      headers: {
        "Content-Type": "application/json",
        ...options.headers,
      },
      ...options,
    });

    if (!response.ok) {
      throw new Error(
        `API request failed: ${response.status} ${response.statusText}`,
      );
    }

    return response.json();
  }

  async get<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: "GET" });
  }

  async post<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: "POST",
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async postForm(
    endpoint: string,
    data: Record<string, string>,
  ): Promise<void> {
    const formData = new URLSearchParams(data);

    const url = `${this.baseUrl}${endpoint}`;
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: formData,
    });

    if (!response.ok) {
      throw new Error(
        `API request failed: ${response.status} ${response.statusText}`,
      );
    }
  }

  async put<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: "PUT",
      body: data ? JSON.stringify(data) : undefined,
    });
  }

  async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: "DELETE" });
  }

  async healthCheck(): Promise<{
    healthy: boolean;
    database?: boolean;
    message?: string;
  }> {
    try {
      const healthData = await this.get<{ database: boolean }>("/health");
      return { healthy: true, database: healthData.database };
    } catch (error) {
      return {
        healthy: false,
        message: error instanceof Error ? error.message : "Unknown error",
      };
    }
  }
}

export const api = new ApiManager();
