'use client';

import { useEffect, useMemo, useState } from 'react';
import { ArrowRight, CheckCircle2, Loader2, ShieldCheck, Timer } from 'lucide-react';
import { cn } from '@/lib/utils';

type StepState = 'pending' | 'active' | 'done';

interface OAuthFlowProps {
  provider: string;
  redirectUri: string;
  scopes: string[];
  onComplete?: () => void;
}

interface FlowStep {
  id: string;
  title: string;
  description: string;
}

const baseSteps: FlowStep[] = [
  {
    id: 'start',
    title: 'Launch consent',
    description: 'Open the provider consent page with state + scopes',
  },
  {
    id: 'callback',
    title: 'Validate callback',
    description: 'Verify state, extract code, capture error strings',
  },
  {
    id: 'exchange',
    title: 'Exchange token',
    description: 'Swap code for access/refresh tokens with PKCE',
  },
  {
    id: 'persist',
    title: 'Persist securely',
    description: 'Write tokens to connector vault + checksum',
  },
];

export default function OAuthFlow({ provider, redirectUri, scopes, onComplete }: OAuthFlowProps) {
  const [stepIndex, setStepIndex] = useState(0);
  const [running, setRunning] = useState(false);

  const steps = useMemo(() => baseSteps, []);

  useEffect(() => {
    if (!running) return;
    if (stepIndex >= steps.length - 1) {
      setRunning(false);
      onComplete?.();
      return;
    }

    const timer = setTimeout(() => setStepIndex((prev) => prev + 1), 900);
    return () => clearTimeout(timer);
  }, [running, stepIndex, steps.length, onComplete, steps]);

  const stepState = (index: number): StepState => {
    if (index < stepIndex) return 'done';
    if (index === stepIndex && running) return 'active';
    return 'pending';
  };

  return (
    <div className="bg-slate-900/60 border border-slate-700 rounded-xl p-5 space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-xs uppercase tracking-wide text-slate-400 mb-1">OAuth Flow</p>
          <h3 className="text-xl font-semibold text-slate-100">{provider} authorization</h3>
          <p className="text-sm text-slate-400">
            Redirects to <span className="text-blue-300">{redirectUri}</span> with the scopes below.
          </p>
        </div>
        <button
          onClick={() => {
            setStepIndex(0);
            setRunning(true);
          }}
          aria-label={`Start OAuth flow for ${provider}`}
          className={cn(
            'inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium',
            running
              ? 'bg-slate-700 text-slate-200 cursor-wait'
              : 'bg-blue-600 hover:bg-blue-500 text-white'
          )}
          disabled={running}
        >
          {running ? <Loader2 className="w-4 h-4 animate-spin" /> : <ShieldCheck className="w-4 h-4" />}
          {running ? 'Authorizing...' : 'Start OAuth'}
        </button>
      </div>

      <div className="flex flex-wrap gap-2 text-xs text-slate-300">
        {scopes.map((scope) => (
          <span key={scope} className="px-2 py-1 rounded bg-slate-800 border border-slate-700">
            {scope}
          </span>
        ))}
      </div>

      <div className="space-y-3">
        {steps.map((step, idx) => {
          const state = stepState(idx);
          return (
            <div
              key={step.id}
              className={cn(
                'flex items-start gap-3 rounded-lg border p-3 transition-colors',
                state === 'done' && 'border-emerald-500/60 bg-emerald-500/5',
                state === 'active' && 'border-blue-500/60 bg-blue-500/5',
                state === 'pending' && 'border-slate-700 bg-slate-800/50'
              )}
            >
              <div className="mt-0.5">
                {state === 'done' && <CheckCircle2 className="w-5 h-5 text-emerald-400" />}
                {state === 'active' && <Loader2 className="w-5 h-5 animate-spin text-blue-400" />}
                {state === 'pending' && <Timer className="w-5 h-5 text-slate-500" />}
              </div>
              <div>
                <div className="flex items-center gap-2 text-slate-100 font-medium">
                  {step.title}
                  {state === 'active' && <ArrowRight className="w-4 h-4 text-blue-300" />}
                </div>
                <p className="text-sm text-slate-400">{step.description}</p>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
