const API_URL = process.env.NEXT_PUBLIC_API;
const CSRF_TOKEN = process.env.NEXT_PUBLIC_CSRF_TOKEN;

interface FetchOptions extends RequestInit {
  params?: Record<string, string | number | boolean>;
}

class FetchClient {
  private static instance: FetchClient;

  private constructor() {}

  static getInstance(): FetchClient {
    if (!this.instance) {
      this.instance = new FetchClient();
    }
    return this.instance;
  }

  async fetch<T>(endpoint: string, options: FetchOptions = {}): Promise<T> {
    const { params, ...fetchOptions } = options;

    const url = new URL(`${API_URL}${endpoint}`);
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        url.searchParams.append(key, String(value));
      });
    }

    const response = await fetch(url, {
      ...fetchOptions,
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
        'csrf-token': CSRF_TOKEN ?? '',
        ...fetchOptions.headers,
      },
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  }

  async get<T>(
    endpoint: string,
    params?: Record<string, string | number>
  ): Promise<T> {
    return this.fetch<T>(endpoint, { params });
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  async post<T>(endpoint: string, data?: any): Promise<T> {
    return this.fetch<T>(endpoint, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }
}

export const api = FetchClient.getInstance();
