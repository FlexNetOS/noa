/**
 * NOA UI Home Page
 *
 * Dynamic context-aware interface for NOA interaction.
 */

export default function Home() {
    return (
        <main className="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
            <div className="container mx-auto px-4 py-16">
                <header className="text-center mb-16">
                    <h1 className="text-6xl font-bold bg-gradient-to-r from-blue-400 via-purple-500 to-pink-500 bg-clip-text text-transparent mb-4">
                        NOA
                    </h1>
                    <p className="text-slate-400 text-xl">
                        Your Autonomous AI Assistant
                    </p>
                </header>

                <div className="max-w-2xl mx-auto">
                    <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl p-8 border border-slate-700">
                        <div className="flex items-center gap-3 mb-6">
                            <div className="w-3 h-3 rounded-full bg-emerald-500 animate-pulse" />
                            <span className="text-emerald-400 font-medium">System Online</span>
                        </div>

                        <div className="space-y-4">
                            <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                                <span className="text-slate-300">Neural Runtime</span>
                                <span className="text-emerald-400">Ready</span>
                            </div>
                            <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                                <span className="text-slate-300">Memory System</span>
                                <span className="text-emerald-400">Active</span>
                            </div>
                            <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                                <span className="text-slate-300">Agent Orchestrator</span>
                                <span className="text-amber-400">Initializing</span>
                            </div>
                            <div className="flex items-center justify-between p-4 bg-slate-900/50 rounded-lg">
                                <span className="text-slate-300">P2P Network</span>
                                <span className="text-slate-500">Offline</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    );
}

