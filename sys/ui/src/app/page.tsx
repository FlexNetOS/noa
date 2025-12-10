'use client';

import { useEffect, useState } from 'react';
import ChatInterface from '@/components/ChatInterface';

/**
 * NOA UI Home Page
 *
 * Dynamic context-aware interface for NOA interaction.
 */

interface SystemStatus {
    status: string;
    version: string;
}

interface ComponentStatus {
    api: boolean;
    database: boolean;
    embedder: boolean;
    agents: boolean;
    p2p: boolean;
}

interface StatusResponse {
    status: string;
    components: ComponentStatus;
}

export default function Home() {
    const [apiStatus, setApiStatus] = useState<SystemStatus | null>(null);
    const [systemStatus, setSystemStatus] = useState<StatusResponse | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchStatus = async () => {
            try {
                const [healthRes, statusRes] = await Promise.all([
                    fetch('http://localhost:3001/health'),
                    fetch('http://localhost:3001/api/v1/status'),
                ]);

                const healthData = await healthRes.json();
                const statusData = await statusRes.json();

                setApiStatus(healthData);
                setSystemStatus(statusData);
            } catch (error) {
                console.error('Failed to fetch status:', error);
            } finally {
                setLoading(false);
            }
        };

        fetchStatus();
        const interval = setInterval(fetchStatus, 5000);
        return () => clearInterval(interval);
    }, []);

    const getStatusColor = (status: boolean) => {
        return status ? 'text-emerald-400' : 'text-slate-500';
    };

    const getStatusText = (status: boolean) => {
        return status ? 'Ready' : 'Offline';
    };

    return (
        <main className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
            <div className="container mx-auto px-4 py-8">
                <header className="text-center mb-12">
                    <h1 className="text-5xl font-bold bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent mb-3">
                        NOA
                    </h1>
                    <p className="text-slate-400 text-lg">
                        Your Autonomous AI Assistant
                    </p>
                </header>

                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
                    <div className="lg:col-span-1">
                        <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-6 border border-slate-700">
                            <div className="flex items-center gap-3 mb-6">
                                <div className={`w-3 h-3 rounded-full ${apiStatus?.status === 'healthy' ? 'bg-emerald-500 animate-pulse' : 'bg-red-500'}`} />
                                <span className={`font-medium ${apiStatus?.status === 'healthy' ? 'text-emerald-400' : 'text-red-400'}`}>
                                    {loading ? 'Connecting...' : apiStatus?.status === 'healthy' ? 'System Online' : 'System Offline'}
                                </span>
                            </div>
                            {apiStatus && (
                                <div className="text-slate-500 text-sm mb-6">
                                    Version {apiStatus.version}
                                </div>
                            )}

                            <div className="space-y-3">
                                <div className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                                    <span className="text-slate-300 text-sm">API Server</span>
                                    <span className={getStatusColor(systemStatus?.components.api ?? false)}>
                                        {getStatusText(systemStatus?.components.api ?? false)}
                                    </span>
                                </div>
                                <div className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                                    <span className="text-slate-300 text-sm">Database</span>
                                    <span className={getStatusColor(systemStatus?.components.database ?? false)}>
                                        {getStatusText(systemStatus?.components.database ?? false)}
                                    </span>
                                </div>
                                <div className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                                    <span className="text-slate-300 text-sm">Neural Runtime</span>
                                    <span className={getStatusColor(systemStatus?.components.embedder ?? false)}>
                                        {getStatusText(systemStatus?.components.embedder ?? false)}
                                    </span>
                                </div>
                                <div className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                                    <span className="text-slate-300 text-sm">Agent Orchestrator</span>
                                    <span className={getStatusColor(systemStatus?.components.agents ?? false)}>
                                        {getStatusText(systemStatus?.components.agents ?? false)}
                                    </span>
                                </div>
                                <div className="flex items-center justify-between p-3 bg-slate-900/50 rounded-lg">
                                    <span className="text-slate-300 text-sm">P2P Network</span>
                                    <span className={getStatusColor(systemStatus?.components.p2p ?? false)}>
                                        {getStatusText(systemStatus?.components.p2p ?? false)}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="lg:col-span-2">
                        <ChatInterface />
                    </div>
                </div>
            </div>
        </main>
    );
}

