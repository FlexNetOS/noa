'use client';

export default function MemorySettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">Memory Settings</h2>
        <p className="text-slate-400 text-sm">Configure memory retention and storage</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Retention Period (days)
          </label>
          <input
            type="number"
            min="1"
            max="365"
            defaultValue="90"
            aria-label="Retention period in days"
            className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Max Memory Size (GB)
          </label>
          <input
            type="number"
            min="1"
            max="100"
            defaultValue="10"
            aria-label="Maximum memory size in gigabytes"
            className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100"
          />
        </div>
      </div>
    </div>
  );
}
