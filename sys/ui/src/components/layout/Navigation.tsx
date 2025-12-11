'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import {
  Home,
  Settings,
  Activity,
  Briefcase,
  Package,
  FileText,
  Shield,
  Cpu,
  Users,
  MessageSquare,
  Gauge,
  PlugZap
} from 'lucide-react';
import { cn } from '@/lib/utils';

interface NavItem {
  href: string;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
}

const navItems: NavItem[] = [
  { href: '/', label: 'Dashboard', icon: Home },
  { href: '/chat', label: 'Chat', icon: MessageSquare },
  { href: '/admin/jobs', label: 'Jobs', icon: Briefcase },
  { href: '/admin/capsules', label: 'Capsules', icon: Package },
  { href: '/admin/artifacts', label: 'Artifacts', icon: FileText },
  { href: '/admin/security', label: 'Security', icon: Shield },
  { href: '/admin/sc-dashboard', label: 'SC Dashboard', icon: Gauge },
  { href: '/admin/models', label: 'Models', icon: Cpu },
  { href: '/admin/crm', label: 'CRM', icon: Users },
  { href: '/activity', label: 'Activity Log', icon: Activity },
  { href: '/settings/connectors', label: 'Connectors', icon: PlugZap },
  { href: '/settings', label: 'Settings', icon: Settings },
];

/**
 * Navigation Component
 *
 * Provides sidebar navigation for the NOA UI application.
 */
export default function Navigation() {
  const pathname = usePathname();

  return (
    <nav className="h-full flex flex-col">
      {/* Logo/Header */}
      <div className="p-6 border-b border-slate-700">
        <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent">
          NOA
        </h1>
        <p className="text-slate-400 text-sm mt-1">
          Autonomous AI Assistant
        </p>
      </div>

      {/* Navigation Items */}
      <div className="flex-1 overflow-y-auto p-4 space-y-1">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = pathname === item.href ||
            (item.href !== '/' && pathname?.startsWith(item.href));

          return (
            <Link
              key={item.href}
              href={item.href}
              aria-label={item.label}
              aria-current={isActive ? 'page' : undefined}
              className={cn(
                'flex items-center gap-3 px-4 py-3 rounded-lg transition-colors',
                'hover:bg-slate-700/50',
                isActive
                  ? 'bg-slate-700/70 text-blue-400'
                  : 'text-slate-300 hover:text-slate-100'
              )}
            >
              <Icon className="w-5 h-5" />
              <span className="font-medium">{item.label}</span>
            </Link>
          );
        })}
      </div>

      {/* Footer */}
      <div className="p-4 border-t border-slate-700">
        <div className="text-xs text-slate-500">
          <div>Version 0.1.0</div>
          <div className="mt-1">© 2025 NOA</div>
        </div>
      </div>
    </nav>
  );
}
