'use client';

import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Building, Users, GraduationCap, Server } from 'lucide-react';
import { Skeleton } from '@/components/ui/skeleton';

export function SysAdminDashboard() {
  const { data: organizations, isLoading: loadingOrgs } = useQuery({
    queryKey: ['organizations'],
    queryFn: async () => {
      const response = await api.get('/api/v1/organizations');
      return response.data;
    },
  });

  const { data: users, isLoading: loadingUsers } = useQuery({
    queryKey: ['users-count'],
    queryFn: async () => {
      const response = await api.get('/api/v1/students');
      return response.data;
    },
  });

  const stats = [
    {
      title: 'Total Organizations',
      value: organizations?.length ?? 0,
      icon: Building,
      color: 'text-blue-600',
      bg: 'bg-blue-50',
      loading: loadingOrgs,
    },
    {
      title: 'Total Students',
      value: users?.length ?? 0,
      icon: GraduationCap,
      color: 'text-green-600',
      bg: 'bg-green-50',
      loading: loadingUsers,
    },
    {
      title: 'System Status',
      value: 'Online',
      icon: Server,
      color: 'text-purple-600',
      bg: 'bg-purple-50',
      loading: false,
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">System Dashboard</h1>
        <p className="text-gray-500">Complete system overview across all organizations</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {stats.map((stat) => {
          const Icon = stat.icon;
          return (
            <Card key={stat.title}>
              <CardHeader className="flex flex-row items-center justify-between pb-2">
                <CardTitle className="text-sm font-medium text-gray-500">
                  {stat.title}
                </CardTitle>
                <div className={`p-2 rounded-lg ${stat.bg}`}>
                  <Icon className={`w-5 h-5 ${stat.color}`} />
                </div>
              </CardHeader>
              <CardContent>
                {stat.loading ? (
                  <Skeleton className="h-8 w-16" />
                ) : (
                  <p className="text-3xl font-bold text-gray-900">{stat.value}</p>
                )}
              </CardContent>
            </Card>
          );
        })}
      </div>
    </div>
  );
}