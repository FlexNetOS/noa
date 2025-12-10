'use client';

import { useEffect, useState, type ReactNode } from 'react';
import { ShieldCheck, KeyRound, Info } from 'lucide-react';

interface PasswordManagerHintProps {
  title?: string;
}

/**
 * UI hint to encourage native password manager usage for device pairing flows.
 * Keeps credentials local (no cloud dependency) while improving UX.
 */
export default function PasswordManagerHint({ title }: PasswordManagerHintProps) {
  const [supportsCredentialAPI, setSupportsCredentialAPI] = useState(false);

  useEffect(() => {
    const supported =
      typeof window !== 'undefined' &&
      typeof navigator !== 'undefined' &&
      'credentials' in navigator;
    setSupportsCredentialAPI(supported);
  }, []);

  return (
    <div className="rounded-xl border border-slate-800 bg-slate-900/70 p-4 shadow-lg">
      <div className="flex items-center gap-3 text-slate-100">
        <ShieldCheck className="h-5 w-5 text-emerald-400" />
        <div>
          <p className="text-sm font-semibold">
            {title ?? 'Use your browser password manager'}
          </p>
          <p className="text-xs text-slate-400">
            Store pairing secrets locally so they auto-fill on future sign-ins.
          </p>
        </div>
      </div>

      <div className="mt-3 grid gap-2 text-xs text-slate-300">
        <HintLine icon={<KeyRound className="h-4 w-4 text-blue-400" />}>
          Save the 6-digit PIN or QR pairing token when prompted. It stays on-device.
        </HintLine>
        <HintLine icon={<Info className="h-4 w-4 text-amber-400" />}>
          Autofill works on <code className="font-mono text-slate-100">https://noa.local</code> and
          other trusted origins configured in NOA.
        </HintLine>
        <HintLine icon={<ShieldCheck className="h-4 w-4 text-emerald-400" />}>
          We never send credentials to cloud services; backup/export is up to you.
        </HintLine>
        {supportsCredentialAPI ? (
          <span className="inline-flex w-fit items-center gap-2 rounded-full bg-emerald-500/10 px-3 py-1 text-[11px] text-emerald-300">
            <ShieldCheck className="h-3 w-3" />
            Browser credential API available
          </span>
        ) : (
          <span className="inline-flex w-fit items-center gap-2 rounded-full bg-amber-500/10 px-3 py-1 text-[11px] text-amber-300">
            <Info className="h-3 w-3" />
            Enable password saving in your browser to auto-fill pairing codes
          </span>
        )}
      </div>
    </div>
  );
}

function HintLine({ children, icon }: { children: ReactNode; icon: ReactNode }) {
  return (
    <div className="flex items-start gap-2 leading-relaxed">
      <span className="mt-0.5">{icon}</span>
      <span>{children}</span>
    </div>
  );
}
