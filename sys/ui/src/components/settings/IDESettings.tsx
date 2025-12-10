'use client';

export default function IDESettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">IDE/Editor Settings</h2>
        <p className="text-slate-400 text-sm">Configure IDE and editor integrations</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Default Editor
          </label>
          <select
            className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100"
            aria-label="Select default editor"
          >
            <option>VS Code</option>
            <option>Cursor</option>
            <option>Neovim</option>
          </select>
        </div>

        <div>
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              className="rounded"
              defaultChecked
              aria-label="Auto-sync settings with IDE"
            />
            <span className="text-sm text-slate-300">Auto-sync settings with IDE</span>
          </label>
        </div>
      </div>
    </div>
  );
}
