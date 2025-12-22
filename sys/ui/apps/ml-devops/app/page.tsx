'use client';

/**
 * Main Dashboard Page - Unified MOE Interface
 * 
 * Features:
 * - MOE (Mixture of Experts) conversational UI
 * - Intelligent agent routing
 * - SONA + DeepCode + Local Inference integration
 * - Goal-based request processing
 */

import { UnifiedChat } from '@/components/moe/unified-chat';
import { Card } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Activity, Sparkles, Code2, Workflow, BookOpen, ChevronRight, Zap } from 'lucide-react';
import { motion } from 'framer-motion';
import Link from 'next/link';

export default function HomePage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50 dark:from-gray-900 dark:via-blue-900/20 dark:to-purple-900/20">
      {/* Header */}
      <header className="sticky top-0 z-50 backdrop-blur-lg bg-white/80 dark:bg-gray-900/80 border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <div className="flex items-center gap-3">
              <motion.div
                initial={{ rotate: 0 }}
                animate={{ rotate: 360 }}
                transition={{ duration: 2, repeat: Infinity, ease: 'linear' }}
                className="w-10 h-10 rounded-lg bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center"
              >
                <Sparkles className="w-6 h-6 text-white" />
              </motion.div>
              <div>
                <h1 className="text-xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                  ML DevOps Platform
                </h1>
                <p className="text-xs text-gray-500">MOE · SONA · DeepCode · Local Inference</p>
              </div>
            </div>
            <div className="flex items-center gap-4">
              <Badge variant="secondary">
                <Activity className="w-3 h-3 mr-1" />
                v0.2.0
              </Badge>
              <Badge variant="outline">
                <Zap className="w-3 h-3 mr-1" />
                Phase E.4
              </Badge>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* Hero Section */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="mb-8 text-center"
        >
          <h2 className="text-4xl font-bold mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
            Unified AI Assistant
          </h2>
          <p className="text-lg text-gray-600 dark:text-gray-400 max-w-2xl mx-auto mb-4">
            Conversational-first interface powered by <strong>MOE (Mixture of Experts)</strong>.
            Your requests are treated as goals and intelligently routed to specialized agents.
          </p>
          <div className="flex items-center justify-center gap-2 text-sm">
            <Badge variant="outline">SONA Orchestration</Badge>
            <Badge variant="outline">DeepCode Agents</Badge>
            <Badge variant="outline">Local Inference</Badge>
            <Badge variant="outline">Dynamic Tools</Badge>
          </div>
        </motion.div>

        {/* Unified Chat Interface */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.1 }}
          className="mb-8"
        >
          <Card className="overflow-hidden border-2 shadow-2xl">
            <div className="h-[600px]">
              <UnifiedChat />
            </div>
          </Card>
        </motion.div>

        <Separator className="my-8" />

        {/* Quick Links */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="mb-8"
        >
          <h3 className="text-2xl font-bold mb-6 text-center">Explore Components</h3>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Link href="/deepcode" className="group">
              <Card className="p-6 h-full transition-all hover:shadow-lg hover:scale-105 cursor-pointer border-2 border-blue-500/20 hover:border-blue-500/40">
                <div className="flex items-center gap-3 mb-3">
                  <div className="p-2 rounded-lg bg-blue-500/10 group-hover:bg-blue-500/20 transition-colors">
                    <Code2 className="w-5 h-5 text-blue-600" />
                  </div>
                  <h4 className="font-semibold group-hover:text-blue-600 transition-colors">DeepCode</h4>
                </div>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  Agentic coding system with document analysis, code generation, and review capabilities
                </p>
                <div className="flex items-center text-sm text-blue-600 font-medium">
                  Try DeepCode
                  <ChevronRight className="w-4 h-4 ml-1 group-hover:translate-x-1 transition-transform" />
                </div>
              </Card>
            </Link>

            <Link href="/sona" className="group">
              <Card className="p-6 h-full transition-all hover:shadow-lg hover:scale-105 cursor-pointer border-2 border-purple-500/20 hover:border-purple-500/40">
                <div className="flex items-center gap-3 mb-3">
                  <div className="p-2 rounded-lg bg-purple-500/10 group-hover:bg-purple-500/20 transition-colors">
                    <Workflow className="w-5 h-5 text-purple-600" />
                  </div>
                  <h4 className="font-semibold group-hover:text-purple-600 transition-colors">SONA Orchestration</h4>
                </div>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  Multi-agent LLM orchestration with 5 execution strategies and 7 built-in tools
                </p>
                <div className="flex items-center text-sm text-purple-600 font-medium">
                  Explore SONA
                  <ChevronRight className="w-4 h-4 ml-1 group-hover:translate-x-1 transition-transform" />
                </div>
              </Card>
            </Link>

            <Link href="/docs" className="group">
              <Card className="p-6 h-full transition-all hover:shadow-lg hover:scale-105 cursor-pointer border-2 border-green-500/20 hover:border-green-500/40">
                <div className="flex items-center gap-3 mb-3">
                  <div className="p-2 rounded-lg bg-green-500/10 group-hover:bg-green-500/20 transition-colors">
                    <BookOpen className="w-5 h-5 text-green-600" />
                  </div>
                  <h4 className="font-semibold group-hover:text-green-600 transition-colors">Documentation</h4>
                </div>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-3">
                  Architecture guide, setup instructions, and Phase 2 task roadmap
                </p>
                <div className="flex items-center text-sm text-green-600 font-medium">
                  Read Docs
                  <ChevronRight className="w-4 h-4 ml-1 group-hover:translate-x-1 transition-transform" />
                </div>
              </Card>
            </Link>
          </div>
        </motion.div>

        {/* MOE Features */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          <h3 className="text-2xl font-bold mb-6 text-center">MOE System Features</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {[
              {
                title: 'Intelligent Routing',
                description: 'Automatically selects the best expert agents for each goal',
                icon: <Zap className="w-4 h-4" />,
              },
              {
                title: 'Goal-Based Processing',
                description: 'Treats requests as goals with intent classification',
                icon: <Sparkles className="w-4 h-4" />,
              },
              {
                title: 'Multi-Agent Workflows',
                description: 'Orchestrates specialized agents for complex tasks',
                icon: <Workflow className="w-4 h-4" />,
              },
              {
                title: 'Dynamic Tools',
                description: 'Runtime tool registration and capability expansion',
                icon: <Code2 className="w-4 h-4" />,
              },
              {
                title: 'Local Inference',
                description: 'Qwen3-1.7B with Rust + Candle integration',
                icon: <Activity className="w-4 h-4" />,
              },
              {
                title: 'Context Awareness',
                description: 'Maintains conversation history and workflow tracking',
                icon: <BookOpen className="w-4 h-4" />,
              },
            ].map((feature, idx) => (
              <motion.div
                key={idx}
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                transition={{ delay: 0.4 + idx * 0.1 }}
              >
                <Card className="p-4 hover:shadow-md transition-shadow">
                  <div className="flex items-start gap-3">
                    <div className="p-2 rounded-lg bg-primary/10">
                      {feature.icon}
                    </div>
                    <div>
                      <h4 className="font-semibold mb-1">{feature.title}</h4>
                      <p className="text-sm text-gray-600 dark:text-gray-400">{feature.description}</p>
                    </div>
                  </div>
                </Card>
              </motion.div>
            ))}
          </div>
        </motion.div>
      </main>

      {/* Footer */}
      <footer className="mt-16 py-8 border-t bg-white/50 dark:bg-gray-900/50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-sm text-gray-600 dark:text-gray-400">
          <p>Built with Next.js 14, TypeScript, Tailwind CSS, and shadcn/ui</p>
          <p className="mt-1">Rust Backend: Candle + Qwen3-1.7B · Ready for Tauri v2 + Dioxus migration</p>
        </div>
      </footer>
    </div>
  );
}
