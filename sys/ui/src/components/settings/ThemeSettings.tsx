'use client';

export default function ThemeSettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">Theme Settings</h2>
        <p className="text-slate-400 text-sm">Customize appearance and theme</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Theme
          </label>
          <select
            className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100"
            aria-label="Select theme"
          >
            <option>Dark</option>
            <option>Light</option>
            <option>Auto</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Accent Color
          </label>
          <div className="flex gap-2">
            {['blue', 'purple', 'pink', 'green', 'orange'].map((color) => (
              <button
                key={color}
                className={`w-10 h-10 rounded-lg bg-${color}-500 hover:ring-2 ring-offset-2 ring-offset-slate-800 ring-${color}-400`}
                aria-label={`Select ${color} accent color`}
                aria-pressed={false}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
