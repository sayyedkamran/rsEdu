'use client';

import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { ClipboardList, Award, DollarSign, GraduationCap } from 'lucide-react';

export function GuardianDashboard() {
  const [selectedChild, setSelectedChild] = useState<string | null>(null);

  // Fetch guardian's children
  const { data: children } = useQuery({
    queryKey: ['guardian-children'],
    queryFn: async () => {
      const response = await api.get('/api/v1/students');
      return response.data;
    },
  });

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Guardian Dashboard</h1>
        <p className="text-gray-500">Monitor your children's progress</p>
      </div>

      {/* Child Selector */}
      {children && children.length > 1 && (
        <div className="max-w-xs">
          <Select onValueChange={setSelectedChild}>
            <SelectTrigger>
              <SelectValue placeholder="Select a child" />
            </SelectTrigger>
            <SelectContent>
              {children.map((child: any) => (
                <SelectItem key={child.id} value={child.id.toString()}>
                  {child.first_name} {child.last_name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      )}

      {/* Child Stats */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">
              Attendance
            </CardTitle>
            <div className="p-2 rounded-lg bg-blue-50">
              <ClipboardList className="w-5 h-5 text-blue-600" />
            </div>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-gray-900">0%</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">
              Latest Result
            </CardTitle>
            <div className="p-2 rounded-lg bg-green-50">
              <Award className="w-5 h-5 text-green-600" />
            </div>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-gray-900">N/A</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">
              Fee Status
            </CardTitle>
            <div className="p-2 rounded-lg bg-orange-50">
              <DollarSign className="w-5 h-5 text-orange-600" />
            </div>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-gray-900">Clear</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between pb-2">
            <CardTitle className="text-sm font-medium text-gray-500">
              Class
            </CardTitle>
            <div className="p-2 rounded-lg bg-purple-50">
              <GraduationCap className="w-5 h-5 text-purple-600" />
            </div>
          </CardHeader>
          <CardContent>
            <p className="text-3xl font-bold text-gray-900">N/A</p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}