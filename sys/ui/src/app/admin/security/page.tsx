'use client';

import { Shield, AlertTriangle, CheckCircle } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

export default function SecurityPage() {
  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <Shield className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">SBOM & Security</h1>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6">
            <h2 className="text-lg font-semibold text-slate-100 mb-4">Security Scan</h2>
            <div className="space-y-3">
              <div className="flex items-center justify-between">
                <span className="text-slate-300">Vulnerabilities</span>
                <span className="flex items-center gap-2 text-red-400">
                  <AlertTriangle className="w-4 h-4" />
                  0
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-slate-300">Dependencies</span>
                <span className="flex items-center gap-2 text-emerald-400">
                  <CheckCircle className="w-4 h-4" />
                  Scanned
                </span>
              </div>
            </div>
          </div>

          <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6">
            <h2 className="text-lg font-semibold text-slate-100 mb-4">SBOM</h2>
            <p className="text-slate-400 text-sm">
              Software Bill of Materials will be displayed here once generated.
            </p>
          </div>
        </div>
      </div>
    </MainLayout>
  );
}

