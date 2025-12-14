'use client';

export default function AISettings() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold text-slate-100 mb-2">AI/LLM Settings</h2>
        <p className="text-slate-400 text-sm">Configure AI model preferences and behavior</p>
      </div>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Default Model
          </label>
          <select className="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100">
            <option>llama.cpp (local)</option>
            <option>Claude Code</option>
            <option>Codex</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-slate-300 mb-2">
            Temperature
          </label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            defaultValue="0.7"
            className="w-full"
          />
        </div>

        <div>
          <label className="flex items-center gap-2">
            <input type="checkbox" className="rounded" />
            <span className="text-sm text-slate-300">Enable streaming responses</span>
          </label>
        </div>
      </div>
    </div>
  );
}

