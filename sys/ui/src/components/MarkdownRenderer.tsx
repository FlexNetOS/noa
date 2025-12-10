'use client';

import { ReactNode } from 'react';

interface MarkdownRendererProps {
  content: string;
}

/**
 * Markdown Renderer Component
 *
 * Renders markdown content with basic formatting support.
 * In a production app, this would use a library like react-markdown.
 */
export default function MarkdownRenderer({ content }: MarkdownRendererProps) {
  // Simple markdown parsing (basic implementation)
  // In production, use react-markdown or similar
  const renderMarkdown = (text: string): ReactNode => {
    const lines = text.split('\n');
    const elements: ReactNode[] = [];

    lines.forEach((line, index) => {
      if (line.trim() === '') {
        elements.push(<br key={index} />);
        return;
      }

      // Code blocks
      if (line.startsWith('```')) {
        elements.push(<code key={index} className="block bg-slate-900 p-2 rounded">{line}</code>);
        return;
      }

      // Inline code
      const codeRegex = /`([^`]+)`/g;
      if (codeRegex.test(line)) {
        const parts = line.split(codeRegex);
        const rendered: ReactNode[] = [];
        parts.forEach((part, partIndex) => {
          if (partIndex % 2 === 1) {
            rendered.push(
              <code key={partIndex} className="bg-slate-900 px-1 rounded text-sm">
                {part}
              </code>
            );
          } else {
            rendered.push(part);
          }
        });
        elements.push(<p key={index}>{rendered}</p>);
        return;
      }

      // Bold
      const boldRegex = /\*\*([^*]+)\*\*/g;
      if (boldRegex.test(line)) {
        const parts = line.split(boldRegex);
        const rendered: ReactNode[] = [];
        parts.forEach((part, partIndex) => {
          if (partIndex % 2 === 1) {
            rendered.push(<strong key={partIndex}>{part}</strong>);
          } else {
            rendered.push(part);
          }
        });
        elements.push(<p key={index}>{rendered}</p>);
        return;
      }

      // Regular text
      elements.push(<p key={index}>{line}</p>);
    });

    return <div>{elements}</div>;
  };

  return <div className="prose prose-invert max-w-none">{renderMarkdown(content)}</div>;
}

