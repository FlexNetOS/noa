'use client';

import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { FileText, ArrowLeft, Github, Book } from 'lucide-react';
import Link from 'next/link';
import { motion } from 'framer-motion';

export default function DocsPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50 dark:from-gray-900 dark:via-blue-900/20 dark:to-purple-900/20">
      {/* Header */}
      <header className="sticky top-0 z-50 backdrop-blur-lg bg-white/80 dark:bg-gray-900/80 border-b">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <Link href="/" className="flex items-center gap-2 text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 transition-colors">
              <ArrowLeft className="w-4 h-4" />
              Back to Platform
            </Link>
            <div className="flex items-center gap-2">
              <FileText className="w-5 h-5 text-blue-500" />
              <span className="font-semibold">Documentation</span>
            </div>
          </div>
        </div>
      </header>

      {/* Content */}
      <main className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="space-y-8"
        >
          {/* Overview */}
          <Card className="p-8">
            <h1 className="text-3xl font-bold mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              Documentation Hub
            </h1>
            <p className="text-gray-600 dark:text-gray-400 mb-6">
              Comprehensive documentation for the Event-Driven ML DevOps Platform. All documentation files are available in the project root.
            </p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <a
                href="/README.md"
                target="_blank"
                rel="noopener noreferrer"
                className="p-4 border rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
              >
                <Book className="w-6 h-6 text-blue-500 mb-2" />
                <h3 className="font-semibold mb-1">README.md</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Architecture overview and event flow diagrams
                </p>
              </a>
              <a
                href="/SETUP.md"
                target="_blank"
                rel="noopener noreferrer"
                className="p-4 border rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
              >
                <Book className="w-6 h-6 text-green-500 mb-2" />
                <h3 className="font-semibold mb-1">SETUP.md</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Build instructions and environment setup
                </p>
              </a>
              <a
                href="/ARCHITECTURE.md"
                target="_blank"
                rel="noopener noreferrer"
                className="p-4 border rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
              >
                <Book className="w-6 h-6 text-purple-500 mb-2" />
                <h3 className="font-semibold mb-1">ARCHITECTURE.md</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Event sourcing patterns and Rust mapping
                </p>
              </a>
              <Link
                href="/phase2-tasks"
                className="p-4 border rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors block"
              >
                <Book className="w-6 h-6 text-orange-500 mb-2" />
                <h3 className="font-semibold mb-1">Phase 2 Tasks</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Roadmap for Tauri + Rust migration
                </p>
              </Link>
            </div>
          </Card>

          {/* Quick Links */}
          <Card className="p-8">
            <h2 className="text-2xl font-bold mb-4">Quick Start</h2>
            <div className="space-y-4">
              <div>
                <h3 className="font-semibold mb-2">1. Explore the Event Simulator</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Use the Event Simulator on the main page to trigger sample events and see the system in action.
                </p>
              </div>
              <div>
                <h3 className="font-semibold mb-2">2. Try the Chat Interface</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Send messages and watch token-by-token streaming. All interactions emit events.
                </p>
              </div>
              <div>
                <h3 className="font-semibold mb-2">3. Replay Event Streams</h3>
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Save your event stream and replay it at different speeds to understand the event flow.
                </p>
              </div>
            </div>
          </Card>
        </motion.div>
      </main>
    </div>
  );
}
