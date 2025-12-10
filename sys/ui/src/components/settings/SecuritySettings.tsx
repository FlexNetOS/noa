'use client';

export default function SecuritySettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">Security Settings</h2>
        <p className="text-slate-400 text-sm">Configure security and authentication</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="flex items-center gap-2">
            <input type="checkbox" className="rounded" defaultChecked />
            <span className="text-sm text-slate-300">Enable two-factor authentication</span>
          </label>
        </div>

        <div>
          <label className="flex items-center gap-2">
            <input type="checkbox" className="rounded" defaultChecked />
            <span className="text-sm text-slate-300">Encrypt data at rest</span>
          </label>
        </div>

        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Session Timeout (minutes)
          </label>
          <input
            type="number"
            min="5"
            max="1440"
            defaultValue="60"
            className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100"
          />
        </div>
      </div>
    </div>
  );
}

