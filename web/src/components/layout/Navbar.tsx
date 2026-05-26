'use client';

import { useAuthStore } from '@/store/auth';
import { useRouter } from 'next/navigation';
import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { Organization, UserProfileResponse } from '@/types';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { LogOut, User, Settings } from 'lucide-react';
import Link from 'next/link';

export function Navbar() {
  const { user, logout } = useAuthStore();
  const router = useRouter();

  // Fetch organization data
  const { data: organization } = useQuery<Organization>({
    queryKey: ['organization', user?.organization_id],
    queryFn: async () => {
      const response = await api.get(`/api/v1/organizations/${user?.organization_id}`);
      return response.data;
    },
    enabled: !!user?.organization_id,
  });

  // Fetch user profile
  const { data: profile } = useQuery<UserProfileResponse>({
    queryKey: ['profile'],
    queryFn: async () => {
      const response = await api.get('/api/v1/users/me');
      return response.data;
    },
  });

  const handleLogout = () => {
    logout();
    router.push('/login');
  };

  // Get initials for avatar fallback
  const getInitials = (username: string) => {
    return username
      .split(' ')
      .map((n) => n[0])
      .join('')
      .toUpperCase()
      .slice(0, 2);
  };

  return (
    <header className="h-16 bg-white border-b border-gray-200 flex items-center justify-between px-6">
      {/* Organization Info */}
      <div className="flex items-center gap-3">
        {organization?.logo_path && (
          <img
            src={`${process.env.NEXT_PUBLIC_API_URL}/uploads/${organization.logo_path}`}
            alt="Organization Logo"
            className="h-8 w-8 object-contain rounded"
          />
        )}
        <div>
          <h2 className="font-semibold text-gray-900">
            {organization?.name || 'rsEdu'}
          </h2>
          {user?.branch_id && (
            <p className="text-xs text-gray-500">Branch ID: {user.branch_id}</p>
          )}
        </div>
      </div>

      {/* User Menu */}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <button className="flex items-center gap-2 hover:opacity-80 transition-opacity">
            <Avatar className="h-8 w-8">
              <AvatarImage src={profile?.profile_picture_url || ''} />
              <AvatarFallback className="bg-blue-600 text-white text-xs">
                {user?.username ? getInitials(user.username) : 'U'}
              </AvatarFallback>
            </Avatar>
            <div className="text-left hidden md:block">
              <p className="text-sm font-medium text-gray-900">{user?.username}</p>
              <p className="text-xs text-gray-500">{user?.role}</p>
            </div>
          </button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-48">
          <DropdownMenuLabel>My Account</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem asChild>
            <Link href="/profile" className="flex items-center gap-2 cursor-pointer">
              <User className="w-4 h-4" />
              Profile
            </Link>
          </DropdownMenuItem>
          <DropdownMenuItem asChild>
            <Link href="/settings" className="flex items-center gap-2 cursor-pointer">
              <Settings className="w-4 h-4" />
              Settings
            </Link>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            onClick={handleLogout}
            className="flex items-center gap-2 cursor-pointer text-red-600"
          >
            <LogOut className="w-4 h-4" />
            Logout
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </header>
  );
}