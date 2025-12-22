'use client';

/**
 * StatusIndicator Widget - System status display
 * 
 * Rust/Dioxus equivalent:
 * - Simple conditional rendering with Dioxus rsx!
 * - Status enum maps directly to Rust enum
 */

import { Card } from '@/components/ui/card';
import {
  CheckCircle,
  XCircle,
  AlertCircle,
  Loader2,
  Activity,
} from 'lucide-react';
import { motion } from 'framer-motion';

type Status = 'idle' | 'processing' | 'success' | 'error' | 'warning';

interface StatusIndicatorProps {
  status: Status;
  message?: string;
  className?: string;
}

const statusConfig = {
  idle: {
    icon: Activity,
    color: 'text-gray-500',
    bgColor: 'bg-gray-50 dark:bg-gray-900',
    borderColor: 'border-gray-200 dark:border-gray-800',
    label: 'Idle',
  },
  processing: {
    icon: Loader2,
    color: 'text-blue-500',
    bgColor: 'bg-blue-50 dark:bg-blue-900/20',
    borderColor: 'border-blue-200 dark:border-blue-800',
    label: 'Processing',
    animate: true,
  },
  success: {
    icon: CheckCircle,
    color: 'text-green-500',
    bgColor: 'bg-green-50 dark:bg-green-900/20',
    borderColor: 'border-green-200 dark:border-green-800',
    label: 'Success',
  },
  error: {
    icon: XCircle,
    color: 'text-red-500',
    bgColor: 'bg-red-50 dark:bg-red-900/20',
    borderColor: 'border-red-200 dark:border-red-800',
    label: 'Error',
  },
  warning: {
    icon: AlertCircle,
    color: 'text-yellow-500',
    bgColor: 'bg-yellow-50 dark:bg-yellow-900/20',
    borderColor: 'border-yellow-200 dark:border-yellow-800',
    label: 'Warning',
  },
};

export function StatusIndicator({
  status,
  message,
  className = '',
}: StatusIndicatorProps) {
  const config = statusConfig[status] ?? statusConfig.idle;
  const Icon = config.icon;
  const shouldAnimate = 'animate' in config && config.animate;

  return (
    <Card
      className={`p-4 border-2 ${config.bgColor} ${config.borderColor} ${className}`}
    >
      <div className="flex items-center gap-3">
        {shouldAnimate ? (
          <motion.div
            animate={{ rotate: 360 }}
            transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
          >
            <Icon className={`w-5 h-5 ${config.color}`} />
          </motion.div>
        ) : (
          <Icon className={`w-5 h-5 ${config.color}`} />
        )}
        <div className="flex-1">
          <div className={`font-semibold ${config.color}`}>{config.label}</div>
          {message && (
            <div className="text-sm text-gray-600 dark:text-gray-400 mt-1">
              {message}
            </div>
          )}
        </div>
      </div>
    </Card>
  );
}
