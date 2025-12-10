//! Spec-Kit Provider Auto-Detection
//!
//! T373: Detect available Spec-Kit providers based on environment and binaries.

import { existsSync } from 'fs';
import { join } from 'path';

export type SpecKitProvider = 'anthropic' | 'openai' | 'azure-openai' | 'ollama' | 'unknown';

export interface ProviderCandidate {
    name: SpecKitProvider;
    available: boolean;
    reason: string;
    binary?: string;
}

const PROVIDER_ENV_HINTS: Array<{ name: SpecKitProvider; env: string; binary?: string }> = [
    { name: 'anthropic', env: 'ANTHROPIC_API_KEY', binary: 'claude' },
    { name: 'openai', env: 'OPENAI_API_KEY', binary: 'codex' },
    { name: 'azure-openai', env: 'AZURE_OPENAI_ENDPOINT' },
    { name: 'ollama', env: 'OLLAMA_HOST', binary: 'ollama' },
];

const binaryExists = (bin: string, noaRoot?: string): boolean => {
    const localPath = noaRoot ? join(noaRoot, 'bin', bin) : null;
    if (localPath && existsSync(localPath)) {
        return true;
    }
    const paths = (process.env.PATH || '').split(pathSeparator());
    return paths.some(p => existsSync(join(p, bin)));
};

const pathSeparator = () => (process.platform === 'win32' ? ';' : ':');

export function detectProviders(noaRoot = process.env.NOA_ROOT || process.cwd()): ProviderCandidate[] {
    return PROVIDER_ENV_HINTS.map(({ name, env, binary }) => {
        const hasEnv = !!process.env[env];
        const hasBinary = binary ? binaryExists(binary, noaRoot) : false;
        const available = hasEnv || hasBinary;
        const reason = available
            ? `Detected via ${hasEnv ? 'env' : 'binary'}`
            : `Missing ${env} and binary${binary ? ` (${binary})` : ''}`;
        return { name, available, reason, binary };
    });
}

export function selectPreferred(candidates: ProviderCandidate[]): ProviderCandidate | null {
    const available = candidates.filter(c => c.available);
    if (available.length === 0) {
        return null;
    }

    const priority: SpecKitProvider[] = ['anthropic', 'openai', 'azure-openai', 'ollama'];
    return (
        available.sort(
            (a, b) => priority.indexOf(a.name) - priority.indexOf(b.name),
        )[0] || null
    );
}
