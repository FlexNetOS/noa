'use client';

import { useState, useCallback } from 'react';
import { DndContext, DragEndEvent, closestCenter } from '@dnd-kit/core';
import { SortableContext, useSortable, verticalListSortingStrategy } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { GripVertical, X } from 'lucide-react';
import { widgetRegistry, type Widget } from './WidgetRegistry';
import { cn } from '@/lib/utils';

export interface WidgetLayout {
  id: string;
  widgetId: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

interface WidgetGridProps {
  layouts: WidgetLayout[];
  onLayoutChange?: (layouts: WidgetLayout[]) => void;
  onRemove?: (widgetId: string) => void;
}

interface SortableWidgetItemProps {
  layout: WidgetLayout;
  widget: Widget;
  onRemove?: (widgetId: string) => void;
}

function SortableWidgetItem({ layout, widget, onRemove }: SortableWidgetItemProps) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: layout.id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={cn(
        'bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-4',
        'hover:border-slate-600 transition-colors',
        isDragging && 'shadow-lg'
      )}
    >
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <div
            {...attributes}
            {...listeners}
            className="cursor-grab active:cursor-grabbing text-slate-400 hover:text-slate-300"
            aria-label={`Reorder widget ${widget.name}`}
            role="button"
            tabIndex={0}
          >
            <GripVertical className="w-5 h-5" />
          </div>
          <h3 className="font-semibold text-slate-100">{widget.name}</h3>
        </div>
        {onRemove && (
          <button
            onClick={() => onRemove(widget.id)}
            className="p-1 hover:bg-slate-700 rounded text-slate-400 hover:text-red-400 transition-colors"
            aria-label="Remove widget"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>
      <div className="text-sm text-slate-400">
        {widget.category} • {widget.defaultSize?.width || 300}×{widget.defaultSize?.height || 200}
      </div>
    </div>
  );
}

/**
 * Widget Grid Component
 *
 * Provides drag-and-drop widget layout management for the dynamic UI.
 */
export default function WidgetGrid({ layouts, onLayoutChange, onRemove }: WidgetGridProps) {
  const [items, setItems] = useState(layouts);

  const handleDragEnd = useCallback((event: DragEndEvent) => {
    const { active, over } = event;

    if (!over || active.id === over.id) {
      return;
    }

    const oldIndex = items.findIndex(item => item.id === active.id);
    const newIndex = items.findIndex(item => item.id === over.id);

    if (oldIndex !== -1 && newIndex !== -1) {
      const newItems = [...items];
      const [removed] = newItems.splice(oldIndex, 1);
      newItems.splice(newIndex, 0, removed);

      setItems(newItems);
      onLayoutChange?.(newItems);
    }
  }, [items, onLayoutChange]);

  const _handleRemove = useCallback((widgetId: string) => {
    const newItems = items.filter(item => item.widgetId !== widgetId);
    setItems(newItems);
    onLayoutChange?.(newItems);
    onRemove?.(widgetId);
  }, [items, onLayoutChange, onRemove]);

  return (
    <DndContext collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <SortableContext items={items.map(item => item.id)} strategy={verticalListSortingStrategy}>
        <div className="space-y-4">
          {items.length === 0 ? (
            <div className="text-center py-12 text-slate-400">
              No widgets configured. Add widgets from the widget registry.
            </div>
          ) : (
            items.map((layout) => {
              const widget = widgetRegistry.get(layout.widgetId);
              if (!widget) return null;

              return (
                <SortableWidgetItem
                  key={layout.id}
                  layout={layout}
                  widget={widget}
                  onRemove={handleRemove}
                />
              );
            })
          )}
        </div>
      </SortableContext>
    </DndContext>
  );
}
