'use client';

export default function SyncSettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">Sync Settings</h2>
        <p className="text-slate-400 text-sm">Configure cross-platform synchronization</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Sync Scope
          </label>
          <select className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100">
            <option>Global</option>
            <option>Per Device</option>
            <option>Per Project</option>
          </select>
        </div>

        <div>
          <label className="flex items-center gap-2">
            <input type="checkbox" className="rounded" defaultChecked />
            <span className="text-sm text-slate-300">Enable automatic sync</span>
          </label>
        </div>
      </div>
    </div>
  );
}

