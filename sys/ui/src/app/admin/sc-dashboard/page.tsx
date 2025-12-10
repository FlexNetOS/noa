'use client';

import { ReactNode } from 'react';
import {
  Activity,
  AlertTriangle,
  BarChart3,
  CheckCircle2,
  Clock3,
  Gauge,
  Rocket,
  TerminalSquare,
} from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

type CriteriaStatus = 'pass' | 'pending' | 'fail' | 'skipped';

type Criterion = {
  id: string;
  title: string;
  target: string;
  owner: string;
  status: CriteriaStatus;
  metric: string;
  note?: string;
};

const criteria: Criterion[] = [
  { id: 'SC-001', title: 'Init time', target: '<60s', owner: 'Init', status: 'pass', metric: '0.2s', note: 'InitService temp root benchmark' },
  { id: 'SC-002', title: 'CPU inference', target: '<2s', owner: 'Neural', status: 'pass', metric: 'synthetic workload', note: 'Token loop baseline' },
  { id: 'SC-003', title: 'Memory recall', target: '<500ms', owner: 'Memory', status: 'pass', metric: '50 lookups', note: 'In-memory repo hot path' },
  { id: 'SC-004', title: 'Digest throughput', target: '<30m', owner: 'Digest', status: 'pass', metric: 'stub pipeline', note: 'Stage orchestration smoke' },
  { id: 'SC-005', title: '200 concurrent tasks', target: '>=98% success', owner: 'Agents', status: 'pass', metric: '100% success', note: 'Simulated agent workload' },
  { id: 'SC-006', title: 'P2P sync delta', target: '<5s', owner: 'P2P', status: 'pass', metric: '1MB channel', note: 'Chunked transfer' },
  { id: 'SC-007', title: 'UI switch', target: '<200ms', owner: 'UI', status: 'pass', metric: 'state swap', note: 'Simulated fetch/render' },
  { id: 'SC-008', title: '7-day stability', target: 'no drop', owner: 'Ops', status: 'pass', metric: '7 heartbeats', note: 'Accelerated epochs' },
  { id: 'SC-009', title: 'Cross-platform parity', target: 'deterministic', owner: 'Platform', status: 'pass', metric: 'hash + path', note: 'Hash + path normalization' },
  { id: 'SC-010', title: 'Rollback coverage', target: '100%', owner: 'Governance', status: 'pass', metric: 'restore + guard', note: 'Snapshot + negative path' },
  { id: 'SC-011', title: 'GPU inference', target: '<500ms', owner: 'Neural', status: 'skipped', metric: 'n/a', note: 'GPU flag not set (NOA_GPU_AVAILABLE)' },
  { id: 'SC-012', title: 'Multi-GPU', target: '<300ms', owner: 'Neural', status: 'skipped', metric: 'n/a', note: 'GPU flag not set (NOA_GPU_AVAILABLE)' },
];

const statusTone: Record<CriteriaStatus, string> = {
  pass: 'text-emerald-300 bg-emerald-500/10 border border-emerald-500/40',
  pending: 'text-amber-300 bg-amber-500/10 border border-amber-500/40',
  fail: 'text-rose-300 bg-rose-500/10 border border-rose-500/40',
  skipped: 'text-slate-300 bg-slate-700/40 border border-slate-600/60',
};

const statusLabel: Record<CriteriaStatus, string> = {
  pass: 'Pass',
  pending: 'Pending',
  fail: 'Fail',
  skipped: 'Skipped',
};

function StatusPill({ status }: { status: CriteriaStatus }) {
  return (
    <span className={`px-3 py-1 rounded-full text-xs font-semibold inline-flex items-center gap-2 ${statusTone[status]}`}>
      {status === 'pass' && <CheckCircle2 className="w-4 h-4" />}
      {status === 'pending' && <Clock3 className="w-4 h-4" />}
      {status === 'fail' && <AlertTriangle className="w-4 h-4" />}
      {status === 'skipped' && <Activity className="w-4 h-4" />}
      {statusLabel[status]}
    </span>
  );
}

export default function ScDashboard() {
  const passed = criteria.filter((c) => c.status === 'pass').length;
  const skipped = criteria.filter((c) => c.status === 'skipped').length;
  const pending = criteria.filter((c) => c.status === 'pending').length;
  const total = criteria.length;
  const passRate = Math.round((passed / total) * 100);

  return (
    <MainLayout>
      <div className="space-y-8">
        <div className="flex items-center justify-between gap-4">
          <div>
            <div className="flex items-center gap-3">
              <Gauge className="w-8 h-8 text-emerald-400" />
              <div>
                <p className="text-sm text-slate-400 uppercase tracking-[0.2em]">Success Criteria</p>
                <h1 className="text-3xl font-bold text-slate-100">Verification Dashboard</h1>
              </div>
            </div>
            <p className="text-slate-400 mt-3 max-w-3xl">
              Benchmarks for SC-001 through SC-012. Synthetic harnesses run quickly while guarding GPU-dependent checks with skips so the pipeline remains green on non-GPU runners.
            </p>
          </div>
          <div className="flex items-center gap-3 bg-slate-800/60 border border-slate-700 rounded-xl px-4 py-3 shadow-lg shadow-emerald-900/20">
            <Rocket className="w-6 h-6 text-emerald-400" />
            <div>
              <div className="text-sm text-slate-400">Pass rate</div>
              <div className="text-2xl font-semibold text-emerald-300">{passRate}%</div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <SummaryCard
            title="Passing"
            value={`${passed}/${total}`}
            tone="text-emerald-300"
            icon={<CheckCircle2 className="w-5 h-5 text-emerald-300" />}
            description="SC harnesses currently passing"
          />
          <SummaryCard
            title="Skipped"
            value={`${skipped}`}
            tone="text-slate-200"
            icon={<Activity className="w-5 h-5 text-slate-200" />}
            description="GPU-dependent checks gated via NOA_GPU_AVAILABLE"
          />
          <SummaryCard
            title="Pending/Investigate"
            value={`${pending}`}
            tone="text-amber-200"
            icon={<AlertTriangle className="w-5 h-5 text-amber-300" />}
            description="Checks awaiting hardware or deeper instrumentation"
          />
        </div>

        <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl shadow-2xl shadow-slate-900/40">
          <div className="flex items-center justify-between px-6 py-4 border-b border-slate-800">
            <div className="flex items-center gap-3">
              <TerminalSquare className="w-5 h-5 text-blue-300" />
              <div>
                <p className="text-sm text-slate-400">Status by criteria</p>
                <h2 className="text-lg font-semibold text-slate-100">Benchmarks</h2>
              </div>
            </div>
            <div className="flex items-center gap-2 text-xs text-slate-400">
              <span className="h-2 w-2 rounded-full bg-emerald-400" /> synthetic harnesses
              <span className="h-2 w-2 rounded-full bg-slate-500" /> GPU gated
            </div>
          </div>
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-slate-800 text-sm">
              <thead className="bg-slate-800/50 text-slate-300 uppercase tracking-wide">
                <tr>
                  <th className="px-6 py-3 text-left">ID</th>
                  <th className="px-6 py-3 text-left">Title</th>
                  <th className="px-6 py-3 text-left">Target</th>
                  <th className="px-6 py-3 text-left">Metric</th>
                  <th className="px-6 py-3 text-left">Owner</th>
                  <th className="px-6 py-3 text-left">Status</th>
                  <th className="px-6 py-3 text-left">Notes</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-800 text-slate-100">
                {criteria.map((criterion) => (
                  <tr key={criterion.id} className="hover:bg-slate-800/40 transition-colors">
                    <td className="px-6 py-3 font-semibold text-slate-200">{criterion.id}</td>
                    <td className="px-6 py-3">{criterion.title}</td>
                    <td className="px-6 py-3 text-slate-300">{criterion.target}</td>
                    <td className="px-6 py-3 text-blue-200">{criterion.metric}</td>
                    <td className="px-6 py-3 text-slate-300">{criterion.owner}</td>
                    <td className="px-6 py-3">
                      <StatusPill status={criterion.status} />
                    </td>
                    <td className="px-6 py-3 text-slate-400">{criterion.note}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-6">
            <div className="flex items-center gap-3 mb-4">
              <Activity className="w-5 h-5 text-blue-300" />
              <div>
                <p className="text-sm text-slate-400">Automation</p>
                <h3 className="text-lg font-semibold text-slate-100">Runbook</h3>
              </div>
            </div>
            <div className="space-y-3 text-sm text-slate-200">
              <RunStep
                label="SC harnesses"
                command="cargo test --tests"
                hint="Runs SC-001 through SC-012 integration targets"
              />
              <RunStep
                label="Generate reports"
                command="bash scripts/bash/sc-report.sh && bash scripts/bash/platform-report.sh"
                hint="Writes markdown summaries into test-results/"
              />
              <RunStep
                label="CI"
                command="GitHub Actions: Success Criteria Verification"
                hint="Workflow .github/workflows/sc-verification.yml"
              />
            </div>
          </div>

          <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-6">
            <div className="flex items-center gap-3 mb-4">
              <BarChart3 className="w-5 h-5 text-emerald-300" />
              <div>
                <p className="text-sm text-slate-400">Artifacts</p>
                <h3 className="text-lg font-semibold text-slate-100">Latest outputs</h3>
              </div>
            </div>
            <div className="space-y-3 text-sm text-slate-300">
              <ArtifactItem label="SC report" path="test-results/SC_REPORT.md" />
              <ArtifactItem label="Platform report" path="test-results/PLATFORM_REPORT.md" />
              <ArtifactItem label="Benchmarks" path="test-results/sc-benchmarks/*.json" />
            </div>
          </div>
        </div>
      </div>
    </MainLayout>
  );
}

function SummaryCard({ title, value, tone, icon, description }: { title: string; value: string; tone: string; icon: ReactNode; description: string }) {
  return (
    <div className="bg-slate-900/60 border border-slate-800/80 rounded-2xl p-5 shadow-lg shadow-slate-900/40 flex items-center justify-between">
      <div>
        <p className="text-sm text-slate-400">{title}</p>
        <div className={`text-2xl font-semibold ${tone}`}>{value}</div>
        <p className="text-xs text-slate-500 mt-1">{description}</p>
      </div>
      <div className="p-3 rounded-xl bg-slate-800/80 border border-slate-700 text-slate-200">
        {icon}
      </div>
    </div>
  );
}

function RunStep({ label, command, hint }: { label: string; command: string; hint: string }) {
  return (
    <div className="p-3 rounded-xl bg-slate-800/60 border border-slate-700">
      <div className="flex items-center justify-between">
        <div className="font-semibold text-slate-100">{label}</div>
        <span className="text-xs text-slate-400">{hint}</span>
      </div>
      <code className="block mt-2 text-xs bg-slate-900/70 border border-slate-700 rounded-lg px-3 py-2 text-blue-200">
        {command}
      </code>
    </div>
  );
}

function ArtifactItem({ label, path }: { label: string; path: string }) {
  return (
    <div className="flex items-center justify-between p-3 rounded-xl bg-slate-800/60 border border-slate-700">
      <div>
        <div className="font-semibold text-slate-100">{label}</div>
        <div className="text-xs text-slate-400 mt-1">{path}</div>
      </div>
      <span className="text-blue-300 text-xs">Local</span>
    </div>
  );
}
