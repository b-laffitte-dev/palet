import { AuthToken } from './types';

// Base API URL - will be configured via environment
const BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

// Storage keys
const TOKEN_KEY = 'palet_access_token';

// Create API client with authentication
export function api() {
  // Get token from storage
  const getToken = (): string | null => {
    const rememberMe = localStorage.getItem('palet_remember_me') === 'true';
    const tokenString = rememberMe 
      ? localStorage.getItem(TOKEN_KEY)
      : sessionStorage.getItem(TOKEN_KEY);
    
    if (!tokenString) return null;
    
    try {
      const token: AuthToken = JSON.parse(tokenString);
      return token.accessToken;
    } catch {
      return null;
    }
  };

  const token = getToken();

  return {
    GET: async <T>(path: string, options?: RequestInit): Promise<{ data: T; status: number }> => {
      const headers = new Headers(options?.headers);
      if (token) {
        headers.append('Authorization', `Bearer ${token}`);
      }
      
      const response = await fetch(`${BASE_URL}${path}`, {
        method: 'GET',
        headers,
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw {
          status: response.status,
          message: errorData.message || 'Request failed',
          code: errorData.code,
        };
      }

      const data = await response.json();
      return { data, status: response.status };
    },

    POST: async <T>(path: string, body?: any, options?: RequestInit): Promise<{ data: T; status: number }> => {
      const headers = new Headers(options?.headers);
      headers.append('Content-Type', 'application/json');
      if (token) {
        headers.append('Authorization', `Bearer ${token}`);
      }

      const response = await fetch(`${BASE_URL}${path}`, {
        method: 'POST',
        headers,
        body: body ? JSON.stringify(body) : undefined,
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw {
          status: response.status,
          message: errorData.message || 'Request failed',
          code: errorData.code,
        };
      }

      const data = await response.json();
      return { data, status: response.status };
    },

    PUT: async <T>(path: string, options?: RequestInit & { body?: any }): Promise<{ data: T; status: number }> => {
      const headers = new Headers(options?.headers);
      headers.append('Content-Type', 'application/json');
      if (token) {
        headers.append('Authorization', `Bearer ${token}`);
      }

      const response = await fetch(`${BASE_URL}${path}`, {
        method: 'PUT',
        headers,
        body: options?.body ? JSON.stringify(options.body) : undefined,
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw {
          status: response.status,
          message: errorData.message || 'Request failed',
          code: errorData.code,
        };
      }

      const data = await response.json();
      return { data, status: response.status };
    },

    PATCH: async <T>(path: string, options?: RequestInit & { body?: any }): Promise<{ data: T; status: number }> => {
      const headers = new Headers(options?.headers);
      headers.append('Content-Type', 'application/json');
      if (token) {
        headers.append('Authorization', `Bearer ${token}`);
      }

      const response = await fetch(`${BASE_URL}${path}`, {
        method: 'PATCH',
        headers,
        body: options?.body ? JSON.stringify(options.body) : undefined,
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw {
          status: response.status,
          message: errorData.message || 'Request failed',
          code: errorData.code,
        };
      }

      const data = await response.json();
      return { data, status: response.status };
    },

    DELETE: async <T>(path: string, options?: RequestInit): Promise<{ data: T; status: number }> => {
      const headers = new Headers(options?.headers);
      if (token) {
        headers.append('Authorization', `Bearer ${token}`);
      }

      const response = await fetch(`${BASE_URL}${path}`, {
        method: 'DELETE',
        headers,
        ...options,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw {
          status: response.status,
          message: errorData.message || 'Request failed',
          code: errorData.code,
        };
      }

      const data = await response.json().catch(() => ({} as T));
      return { data, status: response.status };
    },
  };
}

// Health check endpoint
export async function healthCheck() {
  const response = await fetch(`${BASE_URL}/health`);
  return await response.json();
}

export default api;