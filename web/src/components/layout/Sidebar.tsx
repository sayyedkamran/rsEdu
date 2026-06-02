'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import {
  LayoutDashboard,
  Users,
  GraduationCap,
  BookOpen,
  ClipboardList,
  FileText,
  DollarSign,
  Wallet,
  Receipt,
  Settings,
  ChevronLeft,
  ChevronRight,
  School,
  Building,
  Shield,
  HardDrive,
  ScrollText,
  UserCog,
  Award,
  Baby,
} from 'lucide-react';
import { useState } from 'react';
import { cn } from '@/lib/utils';
import { useAuthStore } from '@/store/auth';

// Menu item type
interface MenuItem {
  title: string;
  href: string;
  icon: any;
}

// Role-based menu configuration
const menuConfig: Record<string, MenuItem[]> = {
  // System Level
  sys_admin: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Organizations', href: '/organizations', icon: Building },
    { title: 'Users', href: '/users', icon: Users },
    { title: 'Roles', href: '/roles', icon: Shield },
    { title: 'Permissions', href: '/permissions', icon: ScrollText },
    { title: 'Audit Logs', href: '/audit-logs', icon: FileText },
    { title: 'Storage', href: '/storage', icon: HardDrive },
    { title: 'Settings', href: '/settings', icon: Settings },
  ],
  sys_super: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Organizations', href: '/organizations', icon: Building },
    { title: 'Users', href: '/users', icon: Users },
    { title: 'Audit Logs', href: '/audit-logs', icon: FileText },
    { title: 'Settings', href: '/settings', icon: Settings },
  ],
  sys_read_only: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Organizations', href: '/organizations', icon: Building },
    { title: 'Audit Logs', href: '/audit-logs', icon: FileText },
  ],
  sys_guest: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
  ],

  // Organization Level
  org_ceo: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Branches', href: '/branches', icon: Building },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Fees', href: '/fees', icon: DollarSign },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Reports', href: '/reports', icon: FileText },
    { title: 'Settings', href: '/settings', icon: Settings },
  ],
  org_admin: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Branches', href: '/branches', icon: Building },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Classes', href: '/classes', icon: School },
    { title: 'Fees', href: '/fees', icon: DollarSign },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Expenses', href: '/expenses', icon: Receipt },
    { title: 'Reports', href: '/reports', icon: FileText },
    { title: 'Settings', href: '/settings', icon: Settings },
  ],
  org_accounts: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Fees', href: '/fees', icon: DollarSign },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Expenses', href: '/expenses', icon: Receipt },
    { title: 'Reports', href: '/reports', icon: FileText },
  ],
  org_hr: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Leaves', href: '/leaves', icon: FileText },
  ],
  org_staff: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Admissions', href: '/admissions', icon: UserCog },
  ],

  // School Level
  s_principal: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Classes', href: '/classes', icon: School },
    { title: 'Subjects', href: '/subjects', icon: BookOpen },
    { title: 'Attendance', href: '/attendance', icon: ClipboardList },
    { title: 'Exams', href: '/exams', icon: FileText },
    { title: 'Results', href: '/results', icon: Award },
    { title: 'Fees', href: '/fees', icon: DollarSign },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Expenses', href: '/expenses', icon: Receipt },
    { title: 'Reports', href: '/reports', icon: FileText },
    { title: 'Settings', href: '/settings', icon: Settings },
  ],
  s_vice_principal: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Classes', href: '/classes', icon: School },
    { title: 'Attendance', href: '/attendance', icon: ClipboardList },
    { title: 'Exams', href: '/exams', icon: FileText },
    { title: 'Results', href: '/results', icon: Award },
    { title: 'Reports', href: '/reports', icon: FileText },
  ],
  s_admin: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Classes', href: '/classes', icon: School },
    { title: 'Attendance', href: '/attendance', icon: ClipboardList },
    { title: 'Fees', href: '/fees', icon: DollarSign },
    { title: 'Expenses', href: '/expenses', icon: Receipt },
    { title: 'Settings', href: '/settings', icon: Settings },
  ],
  s_accounts: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Fees', href: '/fees', icon: DollarSign },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Expenses', href: '/expenses', icon: Receipt },
    { title: 'Reports', href: '/reports', icon: FileText },
  ],
  s_hr: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Salary', href: '/salary', icon: Wallet },
    { title: 'Leaves', href: '/leaves', icon: FileText },
  ],
  s_staff: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Attendance', href: '/attendance', icon: ClipboardList },
    { title: 'Admissions', href: '/admissions', icon: UserCog },
  ],
  s_teacher: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'My Classes', href: '/my-classes', icon: School },
    { title: 'Attendance', href: '/attendance', icon: ClipboardList },
    { title: 'Exams', href: '/exams', icon: FileText },
    { title: 'Results', href: '/results', icon: Award },
  ],
  s_student: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'My Attendance', href: '/my-attendance', icon: ClipboardList },
    { title: 'My Results', href: '/my-results', icon: Award },
    { title: 'My Fees', href: '/my-fees', icon: DollarSign },
  ],
  s_guardian: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Attendance', href: '/guardian/attendance', icon: ClipboardList },
    { title: 'Results', href: '/guardian/results', icon: Award },
    { title: 'Fees', href: '/guardian/fees', icon: DollarSign },
  ],
  s_read_only: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
    { title: 'Students', href: '/students', icon: GraduationCap },
    { title: 'Staff', href: '/staff', icon: Users },
    { title: 'Classes', href: '/classes', icon: School },
  ],
  s_guest: [
    { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
  ],
};

// Default menu for unknown roles
const defaultMenu: MenuItem[] = [
  { title: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
];

export function Sidebar() {
  const pathname = usePathname();
  const [collapsed, setCollapsed] = useState(false);
  const { user } = useAuthStore();

  // Get menu items based on user role
  const menuItems = user?.role
    ? menuConfig[user.role] || defaultMenu
    : defaultMenu;

  return (
    <aside
      className={cn(
        'relative flex flex-col h-screen bg-slate-900 text-white transition-all duration-300',
        collapsed ? 'w-16' : 'w-64'
      )}
    >
      {/* Logo Area */}
      <div className="flex items-center justify-between p-4 border-b border-slate-700">
        {!collapsed && (
          <span className="text-xl font-bold text-white">rsEdu</span>
        )}
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="p-1 rounded-lg hover:bg-slate-700 transition-colors ml-auto"
        >
          {collapsed ? (
            <ChevronRight className="w-5 h-5" />
          ) : (
            <ChevronLeft className="w-5 h-5" />
          )}
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-4">
        <ul className="space-y-1 px-2">
          {menuItems.map((item) => {
            const isActive = pathname === item.href;
            const Icon = item.icon;

            return (
              <li key={item.href}>
                <Link
                  href={item.href}
                  className={cn(
                    'flex items-center gap-3 px-3 py-2.5 rounded-lg transition-colors',
                    isActive
                      ? 'bg-blue-600 text-white'
                      : 'text-slate-300 hover:bg-slate-700 hover:text-white'
                  )}
                >
                  <Icon className="w-5 h-5 shrink-0" />
                  {!collapsed && (
                    <span className="text-sm font-medium">{item.title}</span>
                  )}
                </Link>
              </li>
            );
          })}
        </ul>
      </nav>
    </aside>
  );
}