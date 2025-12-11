'use client';

import React from 'react';

interface ErrorBoundaryProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
  onReset?: () => void;
}

interface ErrorBoundaryState {
  hasError: boolean;
}

/**
 * Error boundary with reset option (T802).
 */
export default class ErrorBoundary extends React.Component<
  ErrorBoundaryProps,
  ErrorBoundaryState
> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError() {
    return { hasError: true };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    // In a full app, route this to logging/telemetry
    console.error('UI error boundary caught:', error, errorInfo);
  }

  handleReset = () => {
    this.setState({ hasError: false });
    this.props.onReset?.();
  };

  render() {
    if (this.state.hasError) {
      return (
        this.props.fallback ?? (
          <div className="p-4 rounded-lg border border-red-600/50 bg-red-900/20 text-red-100 space-y-3">
            <div className="text-sm font-semibold">Something went wrong.</div>
            <button
              onClick={this.handleReset}
              className="px-3 py-2 rounded bg-red-700 text-white text-xs"
              aria-label="Retry after error"
            >
              Try again
            </button>
          </div>
        )
      );
    }
    return this.props.children;
  }
}
