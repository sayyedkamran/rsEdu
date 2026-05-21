import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { AuthResponse } from '@/types';

interface AuthState {
  token: string | null;
  user: {
    username: string;
    role: string;
    organization_id: number | null;
    branch_id: number | null;
  } | null;
  isAuthenticated: boolean;
  login: (data: AuthResponse) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      user: null,
      isAuthenticated: false,

      login: (data: AuthResponse) => {
        localStorage.setItem('token', data.token);
        set({
          token: data.token,
          user: {
            username: data.username,
            role: data.role,
            organization_id: data.organization_id,
            branch_id: data.branch_id,
          },
          isAuthenticated: true,
        });
      },

      logout: () => {
        localStorage.removeItem('token');
        set({
          token: null,
          user: null,
          isAuthenticated: false,
        });
      },
    }),
    {
      name: 'rsedu-auth',
    }
  )
);