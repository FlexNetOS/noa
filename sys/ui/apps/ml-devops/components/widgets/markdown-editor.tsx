'use client';

import React, { useState, useEffect, useCallback } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import
  {
    Bold, Italic, List, ListOrdered, Link2, Image,
    Code, Eye, Edit3, Copy, Download
  } from 'lucide-react';
import { motion } from 'framer-motion';

/**
 * MarkdownEditor Widget - Live markdown editor with preview
 * 
 * Features:
 * - Split view (edit + preview) or tab view
 * - Toolbar with common markdown shortcuts
 * - Live preview rendering
 * - Copy markdown and export
 * - Syntax highlighting in preview
 * - Auto-save support
 * 
 * Rust/Dioxus Translation:
 * - Use pulldown-cmark for markdown parsing
 * - Implement toolbar as Dioxus component
 * - Use textarea with controlled state
 * - Debounce preview updates with tokio
 */

interface MarkdownEditorconfigs
{
  initialValue?: string;
  placeholder?: string;
  height?: string;
  autoSave?: boolean;
  autoSaveDelay?: number;
  onSave?: ( content: string ) => void;
  onChange?: ( content: string ) => void;
}

interface MarkdownEditorProps
{
  configs: MarkdownEditorconfigs;
  className?: string;
}

export function MarkdownEditor ( { configs, className = '' }: MarkdownEditorProps )
{
  const {
    initialValue = '',
    placeholder = 'Start writing markdown...',
    height = '400px',
    autoSave = false,
    autoSaveDelay = 2000,
    onSave,
    onChange,
  } = configs;

  const [ content, setContent ] = useState( initialValue );
  const [ viewMode, setViewMode ] = useState<'edit' | 'preview' | 'split'>( 'split' );
  const [ isSaving, setIsSaving ] = useState( false );
  const textareaRef = React.useRef<HTMLTextAreaElement>( null );

  // Auto-save effect
  useEffect( () =>
  {
    if ( autoSave && onSave && content !== initialValue )
    {
      const timer = setTimeout( () =>
      {
        setIsSaving( true );
        onSave( content );
        setTimeout( () => setIsSaving( false ), 500 );
      }, autoSaveDelay );

      return () => clearTimeout( timer );
    }
  }, [ content, autoSave, autoSaveDelay, onSave, initialValue ] );

  // Handle content change
  const handleChange = ( value: string ) =>
  {
    setContent( value );
    if ( onChange )
    {
      onChange( value );
    }
  };

  // Insert markdown syntax
  const insertMarkdown = useCallback( ( before: string, after: string = '' ) =>
  {
    if ( !textareaRef.current ) return;

    const textarea = textareaRef.current;
    const start = textarea.selectionStart;
    const end = textarea.selectionEnd;
    const selectedText = content.substring( start, end );
    const newText = content.substring( 0, start ) + before + selectedText + after + content.substring( end );

    handleChange( newText );

    // Restore cursor position
    setTimeout( () =>
    {
      textarea.focus();
      textarea.setSelectionRange(
        start + before.length,
        end + before.length
      );
    }, 0 );
  }, [ content ] );

  // Toolbar actions
  const toolbarActions = [
    { icon: Bold, label: 'Bold', action: () => insertMarkdown( '**', '**' ) },
    { icon: Italic, label: 'Italic', action: () => insertMarkdown( '*', '*' ) },
    { icon: Code, label: 'Code', action: () => insertMarkdown( '`', '`' ) },
    { icon: List, label: 'Bullet List', action: () => insertMarkdown( '- ' ) },
    { icon: ListOrdered, label: 'Numbered List', action: () => insertMarkdown( '1. ' ) },
    { icon: Link2, label: 'Link', action: () => insertMarkdown( '[', '](url)' ) },
    { icon: Image, label: 'Image', action: () => insertMarkdown( '![alt](', ')' ) },
  ];

  // Simple markdown to HTML (basic implementation)
  const renderMarkdown = ( md: string ): string =>
  {
    let html = md
      // Headers
      .replace( /^### (.*$)/gim, '<h3 class="text-lg font-semibold mt-4 mb-2">$1</h3>' )
      .replace( /^## (.*$)/gim, '<h2 class="text-xl font-bold mt-6 mb-3">$1</h2>' )
      .replace( /^# (.*$)/gim, '<h1 class="text-2xl font-bold mt-8 mb-4">$1</h1>' )
      // Bold and Italic
      .replace( /\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>' )
      .replace( /\*\*(.+?)\*\*/g, '<strong>$1</strong>' )
      .replace( /\*(.+?)\*/g, '<em>$1</em>' )
      // Code
      .replace( /`([^`]+)`/g, '<code class="px-1.5 py-0.5 bg-muted rounded text-sm font-mono">$1</code>' )
      // Links
      .replace( /\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" class="text-primary underline">$1</a>' )
      // Lists
      .replace( /^\* (.+)$/gim, '<li class="ml-6 list-disc">$1</li>' )
      .replace( /^- (.+)$/gim, '<li class="ml-6 list-disc">$1</li>' )
      .replace( /^\d+\. (.+)$/gim, '<li class="ml-6 list-decimal">$1</li>' )
      // Line breaks
      .replace( /\n/g, '<br />' );

    return html;
  };

  // Copy to clipboard
  const copyToClipboard = () =>
  {
    navigator.clipboard.writeText( content );
  };

  // Download as file
  const downloadMarkdown = () =>
  {
    const blob = new Blob( [ content ], { type: 'text/markdown' } );
    const url = URL.createObjectURL( blob );
    const a = document.createElement( 'a' );
    a.href = url;
    a.download = `document-${ Date.now() }.md`;
    a.click();
    URL.revokeObjectURL( url );
  };

  return (
    <Card className={ className }>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Edit3 className="h-5 w-5" />
            Markdown Editor
          </CardTitle>
          <div className="flex items-center gap-2">
            {/* View Mode Toggles */ }
            <div className="flex border rounded-lg">
              <Button
                variant={ viewMode === 'edit' ? 'default' : 'ghost' }
                size="sm"
                onClick={ () => setViewMode( 'edit' ) }
                className="rounded-r-none"
              >
                <Edit3 className="h-4 w-4" />
              </Button>
              <Button
                variant={ viewMode === 'split' ? 'default' : 'ghost' }
                size="sm"
                onClick={ () => setViewMode( 'split' ) }
                className="rounded-none border-x"
              >
                Split
              </Button>
              <Button
                variant={ viewMode === 'preview' ? 'default' : 'ghost' }
                size="sm"
                onClick={ () => setViewMode( 'preview' ) }
                className="rounded-l-none"
              >
                <Eye className="h-4 w-4" />
              </Button>
            </div>

            {/* Actions */ }
            <Button variant="outline" size="sm" onClick={ copyToClipboard }>
              <Copy className="h-4 w-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={ downloadMarkdown }>
              <Download className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent className="p-0">
        {/* Toolbar */ }
        { ( viewMode === 'edit' || viewMode === 'split' ) && (
          <div className="flex items-center gap-1 px-4 py-2 border-b bg-muted/50">
            { toolbarActions.map( ( action, idx ) => (
              <Button
                key={ idx }
                variant="ghost"
                size="icon"
                className="h-8 w-8"
                onClick={ action.action }
                title={ action.label }
              >
                <action.icon className="h-4 w-4" />
              </Button>
            ) ) }

            { isSaving && (
              <motion.span
                initial={ { opacity: 0 } }
                animate={ { opacity: 1 } }
                className="ml-auto text-xs text-muted-foreground"
              >
                Saving...
              </motion.span>
            ) }
          </div>
        ) }

        {/* Editor/Preview Area */ }
        <div
          className="grid"
          style={ {
            gridTemplateColumns: viewMode === 'split' ? '1fr 1fr' : '1fr',
            height,
          } }
        >
          {/* Editor */ }
          { ( viewMode === 'edit' || viewMode === 'split' ) && (
            <div className={ viewMode === 'split' ? 'border-r' : '' }>
              <textarea
                ref={ textareaRef }
                value={ content }
                onChange={ ( e ) => handleChange( e.target.value ) }
                placeholder={ placeholder }
                className="w-full h-full p-4 resize-none bg-background border-0 focus:outline-none focus:ring-0 font-mono text-sm"
              />
            </div>
          ) }

          {/* Preview */ }
          { ( viewMode === 'preview' || viewMode === 'split' ) && (
            <div className="p-4 overflow-y-auto prose prose-sm dark:prose-invert max-w-none">
              { content ? (
                <div dangerouslySetInnerHTML={ { __html: renderMarkdown( content ) } } />
              ) : (
                <p className="text-muted-foreground italic">Preview will appear here...</p>
              ) }
            </div>
          ) }
        </div>

        {/* Status Bar */ }
        <div className="flex items-center justify-between px-4 py-2 border-t bg-muted/30 text-xs text-muted-foreground">
          <span>{ content.length } characters</span>
          <span>{ content.split( /\s+/ ).filter( Boolean ).length } words</span>
          <span>{ content.split( '\n' ).length } lines</span>
        </div>
      </CardContent>
    </Card>
  );
}
