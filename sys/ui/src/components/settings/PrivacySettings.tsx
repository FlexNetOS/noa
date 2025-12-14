'use client';

export default function PrivacySettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">Privacy Settings</h2>
        <p className="text-slate-400 text-sm">Configure privacy and data handling</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="flex items-center gap-2">
            <input type="checkbox" className="rounded" defaultChecked />
            <span className="text-sm text-slate-300">Share anonymous usage data</span>
          </label>
        </div>

        <div>
          <label className="flex items-center gap-2">
            <input type="checkbox" className="rounded" />
            <span className="text-sm text-slate-300">Allow telemetry collection</span>
          </label>
        </div>
      </div>
    </div>
  );
}

