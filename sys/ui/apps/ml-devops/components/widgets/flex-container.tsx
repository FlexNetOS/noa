'use client';

import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { AlignJustify } from 'lucide-react';

/**
 * FlexContainer Widget - Flexbox layout for nested widgets
 * 
 * Features:
 * - Flexbox-based layout
 * - Horizontal or vertical direction
 * - configsurable alignment and spacing
 * - Support for nested widgets
 * 
 * Rust/Dioxus Translation:
 * - Use flexbox styling in Dioxus
 * - Implement with component children
 * - Map to Dioxus layout props
 */

interface FlexItemconfigs
{
  widgetId: string;
  flex?: number; // flex grow
  order?: number;
}

interface FlexContainerconfigs
{
  title?: string;
  direction?: 'row' | 'column';
  align?: 'start' | 'center' | 'end' | 'stretch';
  justify?: 'start' | 'center' | 'end' | 'between' | 'around';
  gap?: number;
  wrap?: boolean;
  items: FlexItemconfigs[];
}

interface FlexContainerProps
{
  configs: FlexContainerconfigs;
  className?: string;
  children?: React.ReactNode;
}

export function FlexContainer ( { configs, className = '', children }: FlexContainerProps )
{
  const {
    title = 'Flex Layout',
    direction = 'row',
    align = 'stretch',
    justify = 'start',
    gap = 16,
    wrap = false,
    items,
  } = configs;

  // Map alignment values to CSS classes
  const alignMap = {
    start: 'items-start',
    center: 'items-center',
    end: 'items-end',
    stretch: 'items-stretch',
  };

  const justifyMap = {
    start: 'justify-start',
    center: 'justify-center',
    end: 'justify-end',
    between: 'justify-between',
    around: 'justify-around',
  };

  return (
    <Card className={ className }>
      { title && (
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <AlignJustify className="h-5 w-5" />
            { title }
          </CardTitle>
        </CardHeader>
      ) }
      <CardContent>
        <div
          className={ `flex ${ direction === 'column' ? 'flex-col' : 'flex-row' } ${ alignMap[ align ]
            } ${ justifyMap[ justify ] } ${ wrap ? 'flex-wrap' : '' }` }
          style={ { gap: `${ gap }px` } }
        >
          { React.Children.map( children, ( child, index ) =>
          {
            const item = items[ index ];
            if ( !item ) return child;

            return (
              <div
                key={ item.widgetId }
                style={ {
                  flex: item.flex !== undefined ? item.flex : undefined,
                  order: item.order,
                } }
              >
                { child }
              </div>
            );
          } ) }
        </div>
      </CardContent>
    </Card>
  );
}
