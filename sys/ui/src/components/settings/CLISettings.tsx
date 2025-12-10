'use client';

export default function CLISettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">CLI Settings</h2>
        <p className="text-slate-400 text-sm">Configure command-line interface behavior</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              className="rounded"
              defaultChecked
              aria-label="Enable auto-completion"
            />
            <span className="text-sm text-slate-300">Enable auto-completion</span>
          </label>
        </div>

        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Output Format
          </label>
          <select
            className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100"
            aria-label="Select CLI output format"
          >
            <option>JSON</option>
            <option>YAML</option>
            <option>Table</option>
          </select>
        </div>
      </div>
    </div>
  );
}
