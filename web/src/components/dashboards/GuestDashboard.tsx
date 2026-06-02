'use client';

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { GraduationCap, Users, School, DollarSign, Info } from 'lucide-react';
import Link from 'next/link';

export function GuestDashboard() {
  const stats = [
    {
      title: 'Sample Students',
      value: '250',
      icon: GraduationCap,
      color: 'text-blue-600',
      bg: 'bg-blue-50',
    },
    {
      title: 'Sample Staff',
      value: '45',
      icon: Users,
      color: 'text-green-600',
      bg: 'bg-green-50',
    },
    {
      title: 'Sample Classes',
      value: '18',
      icon: School,
      color: 'text-purple-600',
      bg: 'bg-purple-50',
    },
    {
      title: 'Sample Fee Collection',
      value: 'PKR 2.5M',
      icon: DollarSign,
      color: 'text-orange-600',
      bg: 'bg-orange-50',
    },
  ];

  return (
    <div className="space-y-6">
      {/* Demo Banner */}
      <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 flex items-start gap-3">
        <Info className="w-5 h-5 text-blue-600 shrink-0 mt-0.5" />
        <div>
          <p className="font-medium text-blue-900">You are viewing a demo</p>
          <p className="text-sm text-blue-700 mt-1">
            This is sample data to help you explore rsEdu. 
            Contact us to get started with your school.
          </p>
        </div>
      </div>

      <div>
        <h1 className="text-2xl font-bold text-gray-900">Welcome to rsEdu</h1>
        <p className="text-gray-500">School Management System — Demo Dashboard</p>
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
                <p className="text-3xl font-bold text-gray-900">{stat.value}</p>
              </CardContent>
            </Card>
          );
        })}
      </div>

      {/* CTA */}
      <Card>
        <CardContent className="flex items-center justify-between p-6">
          <div>
            <h3 className="font-semibold text-gray-900">Ready to get started?</h3>
            <p className="text-sm text-gray-500 mt-1">
              Contact us to set up rsEdu for your school
            </p>
          </div>
          <Button>Contact Us</Button>
        </CardContent>
      </Card>
    </div>
  );
}