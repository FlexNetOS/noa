'use client';

import { ReactNode } from 'react';
import Navigation from './Navigation';

interface MainLayoutProps {
  children: ReactNode;
}

/**
 * Main Layout Component
 *
 * Provides the primary layout structure with sidebar navigation
 * for the NOA UI application.
 */
export default function MainLayout({ children }: MainLayoutProps) {
  return (
    <div className="flex h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
      {/* Sidebar */}
      <aside className="w-64 bg-slate-800/50 backdrop-blur-sm border-r border-slate-700 flex-shrink-0">
        <Navigation />
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-auto">
        <div className="container mx-auto px-4 py-8">
          {children}
        </div>
      </main>
    </div>
  );
}

