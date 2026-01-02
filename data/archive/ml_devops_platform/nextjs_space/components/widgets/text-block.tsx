'use client';

/**
 * TextBlock Widget - Markdown content rendering
 * 
 * Rust/Dioxus equivalent:
 * - Use pulldown-cmark for markdown parsing
 * - Render as native Dioxus VirtualDom elements
 */

import { useEffect, useState } from 'react';
import { Card } from '@/components/ui/card';
import { FileText } from 'lucide-react';

interface TextBlockProps {
  content: string;
  markdown?: boolean;
  className?: string;
}

export function TextBlock({ content, markdown = true, className = '' }: TextBlockProps) {
  const [rendered, setRendered] = useState<string>('');

  useEffect(() => {
    if (markdown) {
      // Simple markdown rendering (for MVP)
      // In production, use a proper markdown library
      const processedContent = content
        .replace(/^### (.*$)/gim, '<h3 class="text-lg font-semibold mt-4 mb-2">$1</h3>')
        .replace(/^## (.*$)/gim, '<h2 class="text-xl font-bold mt-6 mb-3">$1</h2>')
        .replace(/^# (.*$)/gim, '<h1 class="text-2xl font-bold mt-8 mb-4">$1</h1>')
        .replace(/\*\*(.*?)\*\*/gim, '<strong>$1</strong>')
        .replace(/\*(.*?)\*/gim, '<em>$1</em>')
        .replace(/`(.*?)`/gim, '<code class="bg-gray-100 dark:bg-gray-800 px-1 py-0.5 rounded text-sm">$1</code>')
        .replace(/^- (.*$)/gim, '<li class="ml-4">$1</li>')
        .replace(/\n\n/g, '</p><p class="mb-2">');
      setRendered(`<p class="mb-2">${processedContent}</p>`);
    } else {
      setRendered(content);
    }
  }, [content, markdown]);

  return (
    <Card className={`p-4 ${className}`}>
      <div className="flex items-start gap-3">
        <FileText className="w-5 h-5 text-blue-500 flex-shrink-0 mt-1" />
        <div
          className="prose prose-sm dark:prose-invert max-w-none flex-1"
          dangerouslySetInnerHTML={{ __html: rendered }}
        />
      </div>
    </Card>
  );
}
