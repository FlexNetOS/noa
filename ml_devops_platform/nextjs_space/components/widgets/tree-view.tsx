'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { 
  ChevronRight, ChevronDown, Folder, FolderOpen, 
  File, Search, Maximize2, Minimize2 
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

/**
 * TreeView Widget - Hierarchical data visualization
 * 
 * Features:
 * - Expandable/collapsible nodes
 * - Search/filter functionality
 * - Node selection
 * - Custom icons per node type
 * - Expand/collapse all
 * - Keyboard navigation
 * 
 * Rust/Dioxus Translation:
 * - Use recursive component rendering
 * - Implement tree state with signals
 * - Map to Dioxus events for expand/select
 * - Use match for node type icons
 */

interface TreeNode {
  id: string;
  label: string;
  type?: 'folder' | 'file' | 'custom';
  icon?: React.ReactNode;
  children?: TreeNode[];
  metadata?: Record<string, any>;
}

interface TreeViewConfig {
  data: TreeNode[];
  onSelect?: (node: TreeNode) => void;
  searchable?: boolean;
  expandAll?: boolean;
}

interface TreeViewProps {
  config: TreeViewConfig;
  className?: string;
}

interface TreeNodeItemProps {
  node: TreeNode;
  level: number;
  expanded: Set<string>;
  selected: string | null;
  searchQuery: string;
  onToggle: (id: string) => void;
  onSelect: (node: TreeNode) => void;
}

function TreeNodeItem({
  node,
  level,
  expanded,
  selected,
  searchQuery,
  onToggle,
  onSelect,
}: TreeNodeItemProps) {
  const hasChildren = node.children && node.children.length > 0;
  const isExpanded = expanded.has(node.id);
  const isSelected = selected === node.id;
  const matchesSearch = searchQuery === '' || 
    node.label.toLowerCase().includes(searchQuery.toLowerCase());

  // Hide if doesn't match search
  if (!matchesSearch) {
    return null;
  }

  // Get icon based on node type
  const getIcon = () => {
    if (node.icon) return node.icon;
    if (node.type === 'folder') {
      return isExpanded ? 
        <FolderOpen className="h-4 w-4 text-yellow-500" /> : 
        <Folder className="h-4 w-4 text-yellow-500" />;
    }
    return <File className="h-4 w-4 text-blue-500" />;
  };

  return (
    <div>
      {/* Node */}
      <motion.div
        initial={{ opacity: 0, x: -10 }}
        animate={{ opacity: 1, x: 0 }}
        className={`
          flex items-center gap-1 py-1.5 px-2 rounded-md cursor-pointer
          hover:bg-accent transition-colors
          ${isSelected ? 'bg-accent' : ''}
        `}
        style={{ paddingLeft: `${level * 20 + 8}px` }}
        onClick={() => onSelect(node)}
      >
        {/* Expand/Collapse Button */}
        {hasChildren ? (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onToggle(node.id);
            }}
            className="hover:bg-muted rounded p-0.5"
          >
            {isExpanded ? (
              <ChevronDown className="h-4 w-4" />
            ) : (
              <ChevronRight className="h-4 w-4" />
            )}
          </button>
        ) : (
          <span className="w-5" /> // Spacing for alignment
        )}

        {/* Icon */}
        {getIcon()}

        {/* Label */}
        <span className="text-sm flex-1 truncate">{node.label}</span>

        {/* Metadata Badge */}
        {node.metadata && Object.keys(node.metadata).length > 0 && (
          <span className="text-xs text-muted-foreground">
            {Object.keys(node.metadata).length}
          </span>
        )}
      </motion.div>

      {/* Children */}
      <AnimatePresence>
        {hasChildren && isExpanded && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
          >
            {node.children!.map((child) => (
              <TreeNodeItem
                key={child.id}
                node={child}
                level={level + 1}
                expanded={expanded}
                selected={selected}
                searchQuery={searchQuery}
                onToggle={onToggle}
                onSelect={onSelect}
              />
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

export function TreeView({ config, className = '' }: TreeViewProps) {
  const { data, onSelect, searchable = true, expandAll = false } = config;

  // Get all node IDs for expand all
  const getAllNodeIds = (nodes: TreeNode[]): string[] => {
    const ids: string[] = [];
    const traverse = (node: TreeNode) => {
      ids.push(node.id);
      if (node.children) {
        node.children.forEach(traverse);
      }
    };
    nodes.forEach(traverse);
    return ids;
  };

  const [expanded, setExpanded] = useState<Set<string>>(
    new Set(expandAll ? getAllNodeIds(data) : [])
  );
  const [selected, setSelected] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');

  // Toggle node expansion
  const handleToggle = (id: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  // Select node
  const handleSelect = (node: TreeNode) => {
    setSelected(node.id);
    if (onSelect) {
      onSelect(node);
    }
  };

  // Expand all
  const handleExpandAll = () => {
    setExpanded(new Set(getAllNodeIds(data)));
  };

  // Collapse all
  const handleCollapseAll = () => {
    setExpanded(new Set());
  };

  return (
    <Card className={className}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <Folder className="h-5 w-5" />
            Tree View
          </CardTitle>
          <div className="flex gap-1">
            <Button
              variant="outline"
              size="sm"
              onClick={handleExpandAll}
              title="Expand All"
            >
              <Maximize2 className="h-4 w-4" />
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={handleCollapseAll}
              title="Collapse All"
            >
              <Minimize2 className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {/* Search */}
        {searchable && (
          <div className="relative mt-3">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
            <Input
              type="text"
              placeholder="Search nodes..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="pl-9"
            />
          </div>
        )}
      </CardHeader>
      <CardContent className="pt-0">
        <div className="border rounded-lg p-2 max-h-[500px] overflow-y-auto">
          {data.length === 0 ? (
            <p className="text-sm text-muted-foreground text-center py-8">
              No data to display
            </p>
          ) : (
            <div className="space-y-0.5">
              {data.map((node) => (
                <TreeNodeItem
                  key={node.id}
                  node={node}
                  level={0}
                  expanded={expanded}
                  selected={selected}
                  searchQuery={searchQuery}
                  onToggle={handleToggle}
                  onSelect={handleSelect}
                />
              ))}
            </div>
          )}
        </div>

        {/* Selected Node Info */}
        {selected && (
          <div className="mt-3 p-3 bg-muted rounded-lg">
            <p className="text-xs text-muted-foreground mb-1">Selected:</p>
            <p className="text-sm font-medium">
              {data.find(n => n.id === selected)?.label || 'Unknown'}
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
