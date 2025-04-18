import { isServer } from '@/utils';
import { camelizeKeys, decamelizeKeys } from 'humps';

const PUBLIC_API = process.env.NEXT_PUBLIC_API;
const INERNAL_API = process.env.INTERNAL_API;

const CSRF_TOKEN = process.env.NEXT_PUBLIC_CSRF_TOKEN;

interface FetchOptions extends RequestInit {
  params?: _Params;
}

type _Params = Record<string, string | number | boolean>;

class FetchClient {
  private static instance: FetchClient;

  private constructor() {}

  private getApiUrl() {
    if (!isServer()) {
      // Server-side
      return PUBLIC_API;
    }
    return INERNAL_API;
    // Client-side
  }

  static getInstance(): FetchClient {
    if (!this.instance) {
      this.instance = new FetchClient();
    }
    return this.instance;
  }

  async fetch<T>(endpoint: string, options: FetchOptions = {}): Promise<T> {
    const { params, ...fetchOptions } = options;

    const url = new URL(`${this.getApiUrl()}${endpoint}`);
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

    const json = await response.json();

    return camelizeKeys(json) as T;
  }

  async get<T>(endpoint: string, params?: _Params): Promise<T> {
    return this.fetch<T>(endpoint, { params });
  }

  async post<T>(
    endpoint: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    body?: any,
    options: RequestInit = {}
  ): Promise<T> {
    return this.fetch<T>(endpoint, {
      method: 'POST',
      ...options,
      body: decamelizeKeys(JSON.stringify(body)),
    });
  }
}

export const api = FetchClient.getInstance();
