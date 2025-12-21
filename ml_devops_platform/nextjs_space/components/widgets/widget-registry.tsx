'use client';

/**
 * Widget Registry - Dynamic widget mounting system
 * 
 * Manages widget lifecycle based on events:
 * - WIDGET_MOUNTED -> mount widget
 * - WIDGET_UPDATED -> update widget props (full or partial)
 * - WIDGET_PATCHED -> apply JSON patches (efficient updates)
 * - WIDGET_UNMOUNTED -> remove widget
 * 
 * Rust/Dioxus equivalent:
 * - Use dynamic component rendering with match statements
 * - Store widget state in use_signal HashMap
 * - Apply patches with json-patch crate
 */

import { useEffect, useState } from 'react';
import { applyPatch, Operation } from 'fast-json-patch';
import { useEventStream } from '@/lib/hooks/use-event-stream';
import {
  WidgetMountedEvent,
  WidgetUpdatedEvent,
  WidgetPatchedEvent,
  WidgetUnmountedEvent,
  WidgetConfig,
} from '@/lib/events/types';
import { TextBlock } from './text-block';
import { CodeBlock } from './code-block';
import { StatusIndicator } from './status-indicator';
import { SimpleChart } from './simple-chart';
import { DataTable } from './data-table';
import { Graph } from './graph';
import { ImageViewer } from './image-viewer';
import { VideoPlayer } from './video-player';
import { FileUploader } from './file-uploader';
import { MarkdownEditor } from './markdown-editor';
import { FormBuilder } from './form-builder';
import { TreeView } from './tree-view';
import { GridContainer } from './grid-container';
import { FlexContainer } from './flex-container';
import { TabsContainer } from './tabs-container';
import { motion, AnimatePresence } from 'framer-motion';

interface WidgetInstance {
  id: string;
  config: WidgetConfig;
}

export function WidgetRegistry({ className = '' }: { className?: string }) {
  const [widgets, setWidgets] = useState<Map<string, WidgetInstance>>(new Map());
  const [, stream] = useEventStream();

  useEffect(() => {
    const handler = (event: any) => {
      if (event.type === 'WIDGET_MOUNTED') {
        const e = event as WidgetMountedEvent;
        setWidgets(prev => {
          const next = new Map(prev);
          next.set(e.widgetId, { id: e.widgetId, config: e.config });
          return next;
        });
      } else if (event.type === 'WIDGET_UPDATED') {
        const e = event as WidgetUpdatedEvent;
        setWidgets(prev => {
          const next = new Map(prev);
          const existing = next.get(e.widgetId);
          if (existing) {
            next.set(e.widgetId, {
              ...existing,
              config: {
                ...existing.config,
                ...e.updates,
                props: { ...existing.config.props, ...(e.updates.props ?? {}) },
              },
            });
          }
          return next;
        });
      } else if (event.type === 'WIDGET_PATCHED') {
        // Efficient JSON-patch update
        const e = event as WidgetPatchedEvent;
        setWidgets(prev => {
          const next = new Map(prev);
          const existing = next.get(e.widgetId);
          if (existing) {
            try {
              // Clone config for patching
              const clonedConfig = JSON.parse(JSON.stringify(existing.config));
              // Apply patches (cast to Operation[] for type compatibility)
              const patchedConfig = applyPatch(clonedConfig, e.patch as Operation[]).newDocument;
              next.set(e.widgetId, {
                ...existing,
                config: patchedConfig,
              });
            } catch (error) {
              console.error('Failed to apply widget patch:', error);
            }
          }
          return next;
        });
      } else if (event.type === 'WIDGET_UNMOUNTED') {
        const e = event as WidgetUnmountedEvent;
        setWidgets(prev => {
          const next = new Map(prev);
          next.delete(e.widgetId);
          return next;
        });
      }
    };

    return stream.subscribe(handler);
  }, [stream]);

  // Helper function to render nested widgets
  const renderNestedWidgets = (items: any[] | undefined) => {
    if (!items || items.length === 0) return null;
    
    return items.map((item: any) => {
      const widgetInstance = Array.from(widgets.values()).find((w: WidgetInstance) => w.id === item.widgetId);
      if (!widgetInstance) return null;
      
      return (
        <div key={item.widgetId}>
          {renderWidget(widgetInstance)}
        </div>
      );
    });
  };

  const renderWidget = (instance: WidgetInstance) => {
    const { config } = instance;

    switch (config.type) {
      case 'TextBlock':
        return (
          <TextBlock
            content={config.props?.content ?? ''}
            markdown={config.props?.markdown ?? true}
          />
        );
      case 'CodeBlock':
        return (
          <CodeBlock
            code={config.props?.code ?? ''}
            language={config.props?.language ?? 'typescript'}
            showLineNumbers={config.props?.showLineNumbers ?? true}
          />
        );
      case 'StatusIndicator':
        return (
          <StatusIndicator
            status={config.props?.status ?? 'idle'}
            message={config.props?.message}
          />
        );
      case 'SimpleChart':
        return (
          <SimpleChart
            title={config.props?.title}
            data={config.props?.data ?? []}
            type={config.props?.type ?? 'bar'}
          />
        );
      case 'DataTable':
        return (
          <DataTable
            title={config.props?.title}
            description={config.props?.description}
            columns={config.props?.columns ?? []}
            data={config.props?.data ?? []}
            pageSize={config.props?.pageSize}
            searchable={config.props?.searchable}
          />
        );
      case 'Graph':
        return (
          <Graph
            title={config.props?.title}
            description={config.props?.description}
            nodes={config.props?.nodes ?? []}
            edges={config.props?.edges ?? []}
            width={config.props?.width}
            height={config.props?.height}
          />
        );
      case 'ImageViewer':
        return (
          <ImageViewer
            title={config.props?.title}
            description={config.props?.description}
            src={config.props?.src ?? ''}
            alt={config.props?.alt}
            width={config.props?.width}
            height={config.props?.height}
            downloadable={config.props?.downloadable}
          />
        );
      case 'VideoPlayer':
        return (
          <VideoPlayer
            title={config.props?.title}
            description={config.props?.description}
            src={config.props?.src ?? ''}
            poster={config.props?.poster}
            width={config.props?.width}
            height={config.props?.height}
            autoPlay={config.props?.autoPlay}
            loop={config.props?.loop}
          />
        );
      case 'FileUploader':
        return (
          <FileUploader
            config={{
              maxFiles: config.props?.maxFiles,
              maxSize: config.props?.maxSize,
              accept: config.props?.accept,
              uploadEndpoint: config.props?.uploadEndpoint,
              onUploadComplete: config.props?.onUploadComplete,
            }}
          />
        );
      case 'MarkdownEditor':
        return (
          <MarkdownEditor
            config={{
              initialValue: config.props?.initialValue,
              placeholder: config.props?.placeholder,
              height: config.props?.height,
              autoSave: config.props?.autoSave,
              autoSaveDelay: config.props?.autoSaveDelay,
              onSave: config.props?.onSave,
              onChange: config.props?.onChange,
            }}
          />
        );
      case 'FormBuilder':
        return (
          <FormBuilder
            config={{
              title: config.props?.title,
              fields: config.props?.fields ?? [],
              submitLabel: config.props?.submitLabel,
              onSubmit: config.props?.onSubmit ?? (async () => {}),
              onReset: config.props?.onReset,
            }}
          />
        );
      case 'TreeView':
        return (
          <TreeView
            config={{
              data: config.props?.data ?? [],
              onSelect: config.props?.onSelect,
              searchable: config.props?.searchable,
              expandAll: config.props?.expandAll,
            }}
          />
        );
      case 'GridContainer':
        return (
          <GridContainer
            config={{
              title: config.props?.title,
              columns: config.props?.columns,
              gap: config.props?.gap,
              items: config.props?.items ?? [],
            }}
          >
            {renderNestedWidgets(config.props?.items)}
          </GridContainer>
        );
      case 'FlexContainer':
        return (
          <FlexContainer
            config={{
              title: config.props?.title,
              direction: config.props?.direction,
              align: config.props?.align,
              justify: config.props?.justify,
              gap: config.props?.gap,
              wrap: config.props?.wrap,
              items: config.props?.items ?? [],
            }}
          >
            {renderNestedWidgets(config.props?.items)}
          </FlexContainer>
        );
      case 'TabsContainer':
        return (
          <TabsContainer
            config={{
              title: config.props?.title,
              tabs: config.props?.tabs ?? [],
              defaultTab: config.props?.defaultTab,
            }}
          >
            {renderNestedWidgets(config.props?.tabs?.map((t: any) => ({ widgetId: t.widgetId })))}
          </TabsContainer>
        );
      default:
        return (
          <div className="p-4 border border-red-500 rounded">
            Unknown widget type: {config.type}
          </div>
        );
    }
  };

  const widgetArray = Array.from(widgets.values());

  return (
    <div className={`space-y-4 ${className}`}>
      <AnimatePresence mode="popLayout">
        {widgetArray.map((widget) => (
          <motion.div
            key={widget.id}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.3 }}
          >
            {renderWidget(widget)}
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
