import { useState, useEffect, useCallback, useContext, createContext, ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import { User, AuthState, AuthToken } from '../types';
import { api } from '../api';

// Auth Context
interface AuthContextType extends AuthState {
  login: (email: string, password: string, rememberMe?: boolean) => Promise<void>;
  logout: () => Promise<void>;
  register: (userData: { email: string; password: string; firstName: string; lastName: string }) => Promise<void>;
  refreshToken: (refreshToken: string) => Promise<AuthToken>;
  setUser: (user: User | null) => void;
  clearError: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

// Storage keys
const TOKEN_KEY = 'palet_access_token';
const REFRESH_TOKEN_KEY = 'palet_refresh_token';
const USER_KEY = 'palet_user';
const REMEMBER_ME_KEY = 'palet_remember_me';

// Helper to parse user from storage
const parseUser = (userString: string | null): User | null => {
  if (!userString) return null;
  try {
    return JSON.parse(userString) as User;
  } catch {
    return null;
  }
};

// Helper to parse token from storage
const parseToken = (tokenString: string | null): AuthToken | null => {
  if (!tokenString) return null;
  try {
    return JSON.parse(tokenString) as AuthToken;
  } catch {
    return null;
  }
};

// Token expiration check
const isTokenExpired = (token: AuthToken): boolean => {
  if (!token.expiresIn) return true;
  const expiryTime = new Date().getTime() + (token.expiresIn * 1000);
  return new Date().getTime() > expiryTime;
};

// Clear all auth data
const clearAuthData = () => {
  sessionStorage.removeItem(TOKEN_KEY);
  sessionStorage.removeItem(REFRESH_TOKEN_KEY);
  sessionStorage.removeItem(USER_KEY);
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(REFRESH_TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
  localStorage.removeItem(REMEMBER_ME_KEY);
};

// Auth Provider
export function AuthProvider({ children }: { children: ReactNode }) {
  const navigate = useNavigate();
  
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<AuthToken | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [requires2FA, setRequires2FA] = useState(false);
  const [twoFactorPending, setTwoFactorPending] = useState(false);

  // Initialize auth from storage
  useEffect(() => {
    const initializeAuth = async () => {
      try {
        const rememberMe = localStorage.getItem(REMEMBER_ME_KEY) === 'true';
        const storedToken = rememberMe ? localStorage.getItem(TOKEN_KEY) : sessionStorage.getItem(TOKEN_KEY);
        const storedRefreshToken = rememberMe ? localStorage.getItem(REFRESH_TOKEN_KEY) : sessionStorage.getItem(REFRESH_TOKEN_KEY);
        const storedUser = rememberMe ? localStorage.getItem(USER_KEY) : sessionStorage.getItem(USER_KEY);

        if (storedToken && storedUser) {
          const parsedUser = parseUser(storedUser);
          const parsedToken = parseToken(storedToken);
          
          if (parsedToken && !isTokenExpired(parsedToken)) {
            setUser(parsedUser);
            setToken(parsedToken);
          } else if (storedRefreshToken) {
            try {
              await refreshAccessToken(storedRefreshToken);
            } catch {
              clearAuthData();
            }
          } else {
            clearAuthData();
          }
        }
      } catch {
        clearAuthData();
      } finally {
        setIsLoading(false);
      }
    };

    initializeAuth();
  }, [navigate]);

  // Login function
  const login = useCallback(async (email: string, password: string, rememberMe = false) => {
    setIsLoading(true);
    setError(null);

    try {
      const response = await api().POST<{ user: User; token: AuthToken }>('/auth/login', { email, password, rememberMe });

      const { user: loggedInUser, token: authToken } = response.data;

      if (rememberMe) {
        localStorage.setItem(TOKEN_KEY, JSON.stringify(authToken));
        localStorage.setItem(REFRESH_TOKEN_KEY, authToken.refreshToken);
        localStorage.setItem(USER_KEY, JSON.stringify(loggedInUser));
        localStorage.setItem(REMEMBER_ME_KEY, 'true');
      } else {
        sessionStorage.setItem(TOKEN_KEY, JSON.stringify(authToken));
        sessionStorage.setItem(REFRESH_TOKEN_KEY, authToken.refreshToken);
        sessionStorage.setItem(USER_KEY, JSON.stringify(loggedInUser));
      }

      setUser(loggedInUser);
      setToken(authToken);
      setRequires2FA(false);
      setTwoFactorPending(false);

      const from = '/';
      navigate(from, { replace: true });
    } catch (err: any) {
      setError(err.message || 'Erreur de connexion');
      if (err.code === 'REQUIRES_2FA') {
        setRequires2FA(true);
        setTwoFactorPending(true);
      }
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [navigate]);

  // Logout function
  const logout = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const storedRefreshToken = localStorage.getItem(REFRESH_TOKEN_KEY) || sessionStorage.getItem(REFRESH_TOKEN_KEY);
      
      if (storedRefreshToken) {
        await api().POST('/auth/logout', { refreshToken: storedRefreshToken });
      }
    } catch {
      // Ignore API errors on logout
    } finally {
      clearAuthData();
      setUser(null);
      setToken(null);
      setRequires2FA(false);
      setTwoFactorPending(false);
      navigate('/', { replace: true });
      setIsLoading(false);
    }
  }, [navigate]);

  // Register function
  const register = useCallback(async (userData: { email: string; password: string; firstName: string; lastName: string }) => {
    setIsLoading(true);
    setError(null);

    try {
      await api().POST('/auth/register', userData);
      navigate('/verification-email');
    } catch (err: any) {
      setError(err.message || "Erreur lors de l'inscription");
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [navigate]);

  // Refresh token function
  const refreshAccessToken = useCallback(async (refreshToken: string) => {
    try {
      const response = await api().POST<{ user: User; token: AuthToken }>('/auth/refresh', { refreshToken });

      const { user: refreshedUser, token: newToken } = response.data;
      const rememberMe = localStorage.getItem(REMEMBER_ME_KEY) === 'true';

      if (rememberMe) {
        localStorage.setItem(TOKEN_KEY, JSON.stringify(newToken));
        localStorage.setItem(USER_KEY, JSON.stringify(refreshedUser));
      } else {
        sessionStorage.setItem(TOKEN_KEY, JSON.stringify(newToken));
        sessionStorage.setItem(USER_KEY, JSON.stringify(refreshedUser));
      }

      setUser(refreshedUser);
      setToken(newToken);
      return newToken;
    } catch {
      clearAuthData();
      setUser(null);
      setToken(null);
      throw new Error('Token refresh failed');
    }
  }, []);

  // Clear error function
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  // Auto-refresh token
  useEffect(() => {
    if (!token?.expiresIn) return;

    const expiryTime = new Date().getTime() + (token.expiresIn * 1000);
    const timeUntilExpiry = expiryTime - new Date().getTime();
    const refreshTimeout = timeUntilExpiry - 30000;

    if (refreshTimeout > 0) {
      const timeout = setTimeout(() => {
        const storedRefreshToken = localStorage.getItem(REFRESH_TOKEN_KEY) || sessionStorage.getItem(REFRESH_TOKEN_KEY);
        if (storedRefreshToken) {
          refreshAccessToken(storedRefreshToken);
        }
      }, refreshTimeout);

      return () => clearTimeout(timeout);
    }
  }, [token, refreshAccessToken]);

  const value: AuthContextType = {
    user,
    token,
    isAuthenticated: !!user && !!token,
    isLoading,
    error,
    requires2FA,
    twoFactorPending,
    login,
    logout,
    register,
    refreshToken: refreshAccessToken,
    setUser,
    clearError,
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}

// Custom hook to use auth context
export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
}

// Export for usage
export { AuthContext };
