'use client';

import { useEffect } from 'react';
import { useSession } from 'next-auth/react';
import { useRouter } from 'next/navigation';
import { useAuthStore } from '@/store/auth';

export default function AuthCallbackPage() {
  const { data: session, status } = useSession();
  const router = useRouter();
  const login = useAuthStore((state) => state.login);

  useEffect(() => {
    if (status === 'authenticated' && session) {
      const jwt_token = (session as any).jwt_token;
      if (jwt_token) {
        login({
          token: jwt_token,
          username: session.user?.name || session.user?.email || 'User',
          role: (session as any).role || 's_guest',
          role_title: (session as any).role_title || 'School Guest',
          organization_id: (session as any).organization_id || null,
          branch_id: (session as any).branch_id || null,
        });
        router.push('/dashboard');
      } else {
        router.push('/login');
      }
    } else if (status === 'unauthenticated') {
      router.push('/login');
    }
  }, [session, status]);

  return (
    <div className="min-h-screen flex items-center justify-center">
      <p className="text-gray-500">Signing you in...</p>
    </div>
  );
}