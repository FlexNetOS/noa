'use client';

import { useState } from 'react';

interface ChatMessage {
    role: 'user' | 'assistant';
    content: string;
}

export default function ChatInterface() {
    const [messages, setMessages] = useState<ChatMessage[]>([]);
    const [input, setInput] = useState('');
    const [loading, setLoading] = useState(false);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!input.trim()) return;

        const userMessage: ChatMessage = { role: 'user', content: input };
        setMessages(prev => [...prev, userMessage]);
        setInput('');
        setLoading(true);

        try {
            const response = await fetch('/api/v1/tasks', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ description: input, priority: 'normal' }),
            });
            
            const data = await response.json();
            
            const assistantMessage: ChatMessage = {
                role: 'assistant',
                content: `Task created with ID: ${data.task_id}. Status: ${data.status}`,
            };
            setMessages(prev => [...prev, assistantMessage]);
        } catch (error) {
            console.error('Failed to create task:', error);
            const errorMessage: ChatMessage = {
                role: 'assistant',
                content: 'Failed to process your request. Please try again.',
            };
            setMessages(prev => [...prev, errorMessage]);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex flex-col h-[600px] bg-slate-800/50 backdrop-blur-sm rounded-2xl border border-slate-700">
            <div className="flex-1 overflow-y-auto p-6 space-y-4">
                {messages.length === 0 ? (
                    <div className="text-center text-slate-400 mt-20">
                        <p className="text-lg mb-2">Welcome to NOA</p>
                        <p className="text-sm">Start a conversation or create a task</p>
                    </div>
                ) : (
                    messages.map((msg, idx) => (
                        <div
                            key={idx}
                            className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
                        >
                            <div
                                className={`max-w-[80%] rounded-lg p-4 ${
                                    msg.role === 'user'
                                        ? 'bg-blue-600 text-white'
                                        : 'bg-slate-700 text-slate-100'
                                }`}
                            >
                                {msg.content}
                            </div>
                        </div>
                    ))
                )}
                {loading && (
                    <div className="flex justify-start">
                        <div className="bg-slate-700 text-slate-100 rounded-lg p-4">
                            <div className="flex items-center gap-2">
                                <div className="w-2 h-2 bg-slate-400 rounded-full animate-bounce" />
                                <div className="w-2 h-2 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '0.1s' }} />
                                <div className="w-2 h-2 bg-slate-400 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }} />
                            </div>
                        </div>
                    </div>
                )}
            </div>
            
            <form onSubmit={handleSubmit} className="p-4 border-t border-slate-700">
                <div className="flex gap-2">
                    <input
                        type="text"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        placeholder="Type a message or task..."
                        className="flex-1 bg-slate-900 text-slate-100 rounded-lg px-4 py-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        disabled={loading}
                    />
                    <button
                        type="submit"
                        disabled={loading || !input.trim()}
                        className="bg-blue-600 hover:bg-blue-700 disabled:bg-slate-700 disabled:cursor-not-allowed text-white px-6 py-3 rounded-lg font-medium transition-colors"
                    >
                        Send
                    </button>
                </div>
            </form>
        </div>
    );
}
