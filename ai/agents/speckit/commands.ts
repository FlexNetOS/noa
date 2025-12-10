//! Spec-Kit Command Generation
//!
//! T374: Generates Spec-Kit command wrappers with provider awareness.

import { ProviderCandidate, SpecKitProvider, detectProviders, selectPreferred } from './provider_detect';

export interface SpecKitCommand {
    executable: string;
    args: string[];
    env: Record<string, string>;
    description: string;
}

export interface CommandOptions {
    provider?: SpecKitProvider;
    noaRoot?: string;
}

export const buildSpecKitCommand = (action: 'scan' | 'plan', target: string, options: CommandOptions = {}): SpecKitCommand => {
    const providers = detectProviders(options.noaRoot);
    const preferred = options.provider
        ? providers.find(p => p.name === options.provider && p.available)
        : selectPreferred(providers);

    const providerName = preferred?.name ?? 'unknown';

    const args = ['speckit', action, target, '--provider', providerName];
    const env: Record<string, string> = {};

    if (preferred?.name === 'anthropic' && process.env.ANTHROPIC_API_KEY) {
        env.ANTHROPIC_API_KEY = process.env.ANTHROPIC_API_KEY;
    }
    if (preferred?.name === 'openai' && process.env.OPENAI_API_KEY) {
        env.OPENAI_API_KEY = process.env.OPENAI_API_KEY;
    }
    if (preferred?.name === 'azure-openai' && process.env.AZURE_OPENAI_ENDPOINT) {
        env.AZURE_OPENAI_ENDPOINT = process.env.AZURE_OPENAI_ENDPOINT;
    }

    return {
        executable: preferred?.binary || 'speckit',
        args,
        env,
        description: `Spec-Kit ${action} for ${target} using ${providerName}`,
    };
};

export const planCommands = (target: string, options?: CommandOptions): SpecKitCommand[] => {
    const primary = buildSpecKitCommand('plan', target, options);
    const scan = buildSpecKitCommand('scan', target, options);
    return [scan, primary];
};
