'use client';

import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { LayoutGrid } from 'lucide-react';

/**
 * GridContainer Widget - Grid layout for nested widgets
 * 
 * Features:
 * - CSS Grid-based layout
 * - Configurable columns and gaps
 * - Responsive grid
 * - Support for nested widgets
 * 
 * Rust/Dioxus Translation:
 * - Use CSS grid styling in Dioxus
 * - Implement with component children
 * - Map to flexbox or grid system
 */

interface GridItemConfig {
  widgetId: string;
  colSpan?: number;
  rowSpan?: number;
}

interface GridContainerConfig {
  title?: string;
  columns?: number; // number of columns
  gap?: number; // gap in pixels
  items: GridItemConfig[];
}

interface GridContainerProps {
  config: GridContainerConfig;
  className?: string;
  children?: React.ReactNode;
}

export function GridContainer({ config, className = '', children }: GridContainerProps) {
  const { title = 'Grid Layout', columns = 3, gap = 16, items } = config;

  return (
    <Card className={className}>
      {title && (
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <LayoutGrid className="h-5 w-5" />
            {title}
          </CardTitle>
        </CardHeader>
      )}
      <CardContent>
        <div
          className="grid"
          style={{
            gridTemplateColumns: `repeat(${columns}, 1fr)`,
            gap: `${gap}px`,
          }}
        >
          {React.Children.map(children, (child, index) => {
            const item = items[index];
            if (!item) return child;

            return (
              <div
                key={item.widgetId}
                style={{
                  gridColumn: item.colSpan ? `span ${item.colSpan}` : undefined,
                  gridRow: item.rowSpan ? `span ${item.rowSpan}` : undefined,
                }}
              >
                {child}
              </div>
            );
          })}
        </div>
      </CardContent>
    </Card>
  );
}
