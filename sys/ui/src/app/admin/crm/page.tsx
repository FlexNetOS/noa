'use client';

import { Users } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

export default function CRMPage() {
  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <Users className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">CRM Controls</h1>
        </div>

        <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6">
          <p className="text-slate-400">
            CRM (Customer Relationship Management) controls will be displayed here.
            This section manages relationships and interactions with external entities.
          </p>
        </div>
      </div>
    </MainLayout>
  );
}

