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
import
  {
    WidgetMountedEvent,
    WidgetUpdatedEvent,
    WidgetPatchedEvent,
    WidgetUnmountedEvent,
    Widgetconfigs,
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

interface WidgetInstance
{
  id: string;
  configs: Widgetconfigs;
}

export function WidgetRegistry ( { className = '' }: { className?: string; } )
{
  const [ widgets, setWidgets ] = useState<Map<string, WidgetInstance>>( new Map() );
  const [ , stream ] = useEventStream();

  useEffect( () =>
  {
    const handler = ( event: any ) =>
    {
      if ( event.type === 'WIDGET_MOUNTED' )
      {
        const e = event as WidgetMountedEvent;
        setWidgets( prev =>
        {
          const next = new Map( prev );
          next.set( e.widgetId, { id: e.widgetId, configs: e.configs } );
          return next;
        } );
      } else if ( event.type === 'WIDGET_UPDATED' )
      {
        const e = event as WidgetUpdatedEvent;
        setWidgets( prev =>
        {
          const next = new Map( prev );
          const existing = next.get( e.widgetId );
          if ( existing )
          {
            next.set( e.widgetId, {
              ...existing,
              configs: {
                ...existing.configs,
                ...e.updates,
                props: { ...existing.configs.props, ...( e.updates.props ?? {} ) },
              },
            } );
          }
          return next;
        } );
      } else if ( event.type === 'WIDGET_PATCHED' )
      {
        // Efficient JSON-patch update
        const e = event as WidgetPatchedEvent;
        setWidgets( prev =>
        {
          const next = new Map( prev );
          const existing = next.get( e.widgetId );
          if ( existing )
          {
            try
            {
              // Clone configs for patching
              const clonedconfigs = JSON.parse( JSON.stringify( existing.configs ) );
              // Apply patches (cast to Operation[] for type compatibility)
              const patchedconfigs = applyPatch( clonedconfigs, e.patch as Operation[] ).newDocument;
              next.set( e.widgetId, {
                ...existing,
                configs: patchedconfigs,
              } );
            } catch ( error )
            {
              console.error( 'Failed to apply widget patch:', error );
            }
          }
          return next;
        } );
      } else if ( event.type === 'WIDGET_UNMOUNTED' )
      {
        const e = event as WidgetUnmountedEvent;
        setWidgets( prev =>
        {
          const next = new Map( prev );
          next.delete( e.widgetId );
          return next;
        } );
      }
    };

    return stream.subscribe( handler );
  }, [ stream ] );

  // Helper function to render nested widgets
  const renderNestedWidgets = ( items: any[] | undefined ) =>
  {
    if ( !items || items.length === 0 ) return null;

    return items.map( ( item: any ) =>
    {
      const widgetInstance = Array.from( widgets.values() ).find( ( w: WidgetInstance ) => w.id === item.widgetId );
      if ( !widgetInstance ) return null;

      return (
        <div key={ item.widgetId }>
          { renderWidget( widgetInstance ) }
        </div>
      );
    } );
  };

  const renderWidget = ( instance: WidgetInstance ) =>
  {
    const { configs } = instance;

    switch ( configs.type )
    {
      case 'TextBlock':
        return (
          <TextBlock
            content={ configs.props?.content ?? '' }
            markdown={ configs.props?.markdown ?? true }
          />
        );
      case 'CodeBlock':
        return (
          <CodeBlock
            code={ configs.props?.code ?? '' }
            language={ configs.props?.language ?? 'typescript' }
            showLineNumbers={ configs.props?.showLineNumbers ?? true }
          />
        );
      case 'StatusIndicator':
        return (
          <StatusIndicator
            status={ configs.props?.status ?? 'idle' }
            message={ configs.props?.message }
          />
        );
      case 'SimpleChart':
        return (
          <SimpleChart
            title={ configs.props?.title }
            data={ configs.props?.data ?? [] }
            type={ configs.props?.type ?? 'bar' }
          />
        );
      case 'DataTable':
        return (
          <DataTable
            title={ configs.props?.title }
            description={ configs.props?.description }
            columns={ configs.props?.columns ?? [] }
            data={ configs.props?.data ?? [] }
            pageSize={ configs.props?.pageSize }
            searchable={ configs.props?.searchable }
          />
        );
      case 'Graph':
        return (
          <Graph
            title={ configs.props?.title }
            description={ configs.props?.description }
            nodes={ configs.props?.nodes ?? [] }
            edges={ configs.props?.edges ?? [] }
            width={ configs.props?.width }
            height={ configs.props?.height }
          />
        );
      case 'ImageViewer':
        return (
          <ImageViewer
            title={ configs.props?.title }
            description={ configs.props?.description }
            src={ configs.props?.src ?? '' }
            alt={ configs.props?.alt }
            width={ configs.props?.width }
            height={ configs.props?.height }
            downloadable={ configs.props?.downloadable }
          />
        );
      case 'VideoPlayer':
        return (
          <VideoPlayer
            title={ configs.props?.title }
            description={ configs.props?.description }
            src={ configs.props?.src ?? '' }
            poster={ configs.props?.poster }
            width={ configs.props?.width }
            height={ configs.props?.height }
            autoPlay={ configs.props?.autoPlay }
            loop={ configs.props?.loop }
          />
        );
      case 'FileUploader':
        return (
          <FileUploader
            configs={ {
              maxFiles: configs.props?.maxFiles,
              maxSize: configs.props?.maxSize,
              accept: configs.props?.accept,
              uploadEndpoint: configs.props?.uploadEndpoint,
              onUploadComplete: configs.props?.onUploadComplete,
            } }
          />
        );
      case 'MarkdownEditor':
        return (
          <MarkdownEditor
            configs={ {
              initialValue: configs.props?.initialValue,
              placeholder: configs.props?.placeholder,
              height: configs.props?.height,
              autoSave: configs.props?.autoSave,
              autoSaveDelay: configs.props?.autoSaveDelay,
              onSave: configs.props?.onSave,
              onChange: configs.props?.onChange,
            } }
          />
        );
      case 'FormBuilder':
        return (
          <FormBuilder
            configs={ {
              title: configs.props?.title,
              fields: configs.props?.fields ?? [],
              submitLabel: configs.props?.submitLabel,
              onSubmit: configs.props?.onSubmit ?? ( async () => { } ),
              onReset: configs.props?.onReset,
            } }
          />
        );
      case 'TreeView':
        return (
          <TreeView
            configs={ {
              data: configs.props?.data ?? [],
              onSelect: configs.props?.onSelect,
              searchable: configs.props?.searchable,
              expandAll: configs.props?.expandAll,
            } }
          />
        );
      case 'GridContainer':
        return (
          <GridContainer
            configs={ {
              title: configs.props?.title,
              columns: configs.props?.columns,
              gap: configs.props?.gap,
              items: configs.props?.items ?? [],
            } }
          >
            { renderNestedWidgets( configs.props?.items ) }
          </GridContainer>
        );
      case 'FlexContainer':
        return (
          <FlexContainer
            configs={ {
              title: configs.props?.title,
              direction: configs.props?.direction,
              align: configs.props?.align,
              justify: configs.props?.justify,
              gap: configs.props?.gap,
              wrap: configs.props?.wrap,
              items: configs.props?.items ?? [],
            } }
          >
            { renderNestedWidgets( configs.props?.items ) }
          </FlexContainer>
        );
      case 'TabsContainer':
        return (
          <TabsContainer
            configs={ {
              title: configs.props?.title,
              tabs: configs.props?.tabs ?? [],
              defaultTab: configs.props?.defaultTab,
            } }
          >
            { renderNestedWidgets( configs.props?.tabs?.map( ( t: any ) => ( { widgetId: t.widgetId } ) ) ) }
          </TabsContainer>
        );
      default:
        return (
          <div className="p-4 border border-red-500 rounded">
            Unknown widget type: { configs.type }
          </div>
        );
    }
  };

  const widgetArray = Array.from( widgets.values() );

  return (
    <div className={ `space-y-4 ${ className }` }>
      <AnimatePresence mode="popLayout">
        { widgetArray.map( ( widget ) => (
          <motion.div
            key={ widget.id }
            initial={ { opacity: 0, y: 20 } }
            animate={ { opacity: 1, y: 0 } }
            exit={ { opacity: 0, y: -20 } }
            transition={ { duration: 0.3 } }
          >
            { renderWidget( widget ) }
          </motion.div>
        ) ) }
      </AnimatePresence>
    </div>
  );
}
