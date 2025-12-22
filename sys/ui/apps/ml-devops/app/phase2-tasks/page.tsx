'use client';

import { Card } from '@/components/ui/card';
import { ArrowLeft, Download } from 'lucide-react';
import Link from 'next/link';
import { motion } from 'framer-motion';
import { useEffect, useState } from 'react';

export default function Phase2TasksPage() {
  const [csvContent, setCsvContent] = useState<string>('');
  const [tasks, setTasks] = useState<any[]>([]);

  useEffect(() => {
    fetch('/phase2_tasks.csv')
      .then(res => res.text())
      .then(text => {
        setCsvContent(text);
        
        // Parse CSV
        const lines = text.split('\n');
        const headers = lines[0]?.split(',') ?? [];
        const taskData = lines.slice(1).filter(line => line.trim()).map(line => {
          const values = line.split(',');
          return {
            task: values[0] ?? '',
            priority: values[1] ?? '',
            complexity: values[2] ?? '',
            dependencies: values[3] ?? '',
            description: values[4] ?? '',
          };
        });
        setTasks(taskData);
      })
      .catch(err => console.error('Failed to load CSV:', err));
  }, []);

  const downloadCSV = () => {
    const blob = new Blob([csvContent], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'phase2_tasks.csv';
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-purple-50 dark:from-gray-900 dark:via-blue-900/20 dark:to-purple-900/20">
      {/* Header */}
      <header className="sticky top-0 z-50 backdrop-blur-lg bg-white/80 dark:bg-gray-900/80 border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex items-center justify-between h-16">
            <Link href="/docs" className="flex items-center gap-2 text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 transition-colors">
              <ArrowLeft className="w-4 h-4" />
              Back to Documentation
            </Link>
            <button
              onClick={downloadCSV}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
            >
              <Download className="w-4 h-4" />
              Download CSV
            </button>
          </div>
        </div>
      </header>

      {/* Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="space-y-8"
        >
          <Card className="p-8">
            <h1 className="text-3xl font-bold mb-4 bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              Phase 2 Tasks - Tauri + Rust Migration
            </h1>
            <p className="text-gray-600 dark:text-gray-400 mb-6">
              Comprehensive roadmap for porting the event-driven ML DevOps platform to Tauri v2 + Dioxus with Rust-based ML inference.
            </p>
            <div className="mb-4 text-sm text-gray-600 dark:text-gray-400">
              Total Tasks: <strong>{tasks.length}</strong>
            </div>
          </Card>

          {/* Task List */}
          <div className="space-y-4">
            {tasks.map((task, index) => (
              <motion.div
                key={index}
                initial={{ opacity: 0, x: -20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ delay: index * 0.02 }}
              >
                <Card className="p-6 hover:shadow-lg transition-shadow">
                  <div className="flex items-start justify-between mb-3">
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
                      {task.task}
                    </h3>
                    <div className="flex gap-2">
                      <span className={`px-2 py-1 text-xs font-medium rounded ${
                        task.priority === 'High' ? 'bg-red-100 text-red-700 dark:bg-red-900/20 dark:text-red-300' :
                        task.priority === 'Medium' ? 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/20 dark:text-yellow-300' :
                        'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300'
                      }`}>
                        {task.priority}
                      </span>
                      <span className={`px-2 py-1 text-xs font-medium rounded ${
                        task.complexity === 'High' ? 'bg-purple-100 text-purple-700 dark:bg-purple-900/20 dark:text-purple-300' :
                        task.complexity === 'Medium' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/20 dark:text-blue-300' :
                        'bg-green-100 text-green-700 dark:bg-green-900/20 dark:text-green-300'
                      }`}>
                        {task.complexity}
                      </span>
                    </div>
                  </div>
                  {task.dependencies !== 'None' && (
                    <div className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                      <strong>Dependencies:</strong> {task.dependencies}
                    </div>
                  )}
                  <p className="text-sm text-gray-700 dark:text-gray-300">
                    {task.description}
                  </p>
                </Card>
              </motion.div>
            ))}
          </div>
        </motion.div>
      </main>
    </div>
  );
}
