'use client';

import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { School, ClipboardList, FileText, Award } from 'lucide-react';
import { Skeleton } from '@/components/ui/skeleton';

export function TeacherDashboard() {
  const { data: classes, isLoading: loadingClasses } = useQuery({
    queryKey: ['classes'],
    queryFn: async () => {
      const response = await api.get('/api/v1/classes');
      return response.data;
    },
  });

  const stats = [
    {
      title: 'My Classes',
      value: classes?.length ?? 0,
      icon: School,
      color: 'text-blue-600',
      bg: 'bg-blue-50',
      loading: loadingClasses,
    },
    {
      title: 'Attendance Today',
      value: 'Not Marked',
      icon: ClipboardList,
      color: 'text-orange-600',
      bg: 'bg-orange-50',
      loading: false,
    },
    {
      title: 'Upcoming Exams',
      value: 0,
      icon: FileText,
      color: 'text-purple-600',
      bg: 'bg-purple-50',
      loading: false,
    },
    {
      title: 'Results Pending',
      value: 0,
      icon: Award,
      color: 'text-green-600',
      bg: 'bg-green-50',
      loading: false,
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Teacher Dashboard</h1>
        <p className="text-gray-500">Your classes and daily tasks</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
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