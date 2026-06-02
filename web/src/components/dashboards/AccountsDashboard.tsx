'use client';

import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { DollarSign, Wallet, Receipt, AlertCircle } from 'lucide-react';
import { Skeleton } from '@/components/ui/skeleton';

export function AccountsDashboard() {
  const { data: feeBills, isLoading: loadingBills } = useQuery({
    queryKey: ['fee-bills'],
    queryFn: async () => {
      const response = await api.get('/api/v1/fee-bills');
      return response.data;
    },
  });

  const pendingBills = feeBills?.filter(
    (bill: any) => bill.status === 'generated' || bill.status === 'overdue'
  ).length ?? 0;

  const stats = [
    {
      title: 'Fee Collection Today',
      value: 'PKR 0',
      icon: DollarSign,
      color: 'text-green-600',
      bg: 'bg-green-50',
      loading: false,
    },
    {
      title: 'Pending Bills',
      value: pendingBills,
      icon: AlertCircle,
      color: 'text-orange-600',
      bg: 'bg-orange-50',
      loading: loadingBills,
    },
    {
      title: 'Salary This Month',
      value: 'PKR 0',
      icon: Wallet,
      color: 'text-blue-600',
      bg: 'bg-blue-50',
      loading: false,
    },
    {
      title: 'Expenses This Month',
      value: 'PKR 0',
      icon: Receipt,
      color: 'text-purple-600',
      bg: 'bg-purple-50',
      loading: false,
    },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Accounts Dashboard</h1>
        <p className="text-gray-500">Financial overview and collections</p>
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