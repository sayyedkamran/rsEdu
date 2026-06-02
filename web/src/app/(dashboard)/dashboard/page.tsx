'use client';

import { useAuthStore } from '@/store/auth';
import { SysAdminDashboard } from '@/components/dashboards/SysAdminDashboard';
import { OrgDashboard } from '@/components/dashboards/OrgDashboard';
import { SchoolDashboard } from '@/components/dashboards/SchoolDashboard';
import { AccountsDashboard } from '@/components/dashboards/AccountsDashboard';
import { TeacherDashboard } from '@/components/dashboards/TeacherDashboard';
import { GuardianDashboard } from '@/components/dashboards/GuardianDashboard';
import { GuestDashboard } from '@/components/dashboards/GuestDashboard';

export default function DashboardPage() {
  const { user } = useAuthStore();
  const role = user?.role ?? '';

  // System level
  if (['sys_admin', 'sys_super'].includes(role)) {
    return <SysAdminDashboard />;
  }

  if (['sys_read_only', 'sys_guest'].includes(role)) {
    return <GuestDashboard />;
  }

  // Organization level
  if (['org_ceo', 'org_admin', 'org_super', 'org_it'].includes(role)) {
    return <OrgDashboard />;
  }

  if (['org_accounts', 'org_hr'].includes(role)) {
    return <AccountsDashboard />;
  }

  if (['org_staff', 'org_read_only'].includes(role)) {
    return <OrgDashboard />;
  }

  if (role === 'org_guest') {
    return <GuestDashboard />;
  }

  // School level
  if (['s_principal', 's_vice_principal', 's_admin', 's_super'].includes(role)) {
    return <SchoolDashboard />;
  }

  if (['s_accounts', 's_hr'].includes(role)) {
    return <AccountsDashboard />;
  }

  if (['s_staff', 's_it', 's_read_only'].includes(role)) {
    return <SchoolDashboard />;
  }

  if (role === 's_teacher') {
    return <TeacherDashboard />;
  }

  if (role === 's_student') {
    return <SchoolDashboard />;
  }

  if (role === 's_guardian') {
    return <GuardianDashboard />;
  }

  if (['s_guest', 's_librarian', 's_transport'].includes(role)) {
    return <GuestDashboard />;
  }

  // Default fallback
  return <SchoolDashboard />;
}