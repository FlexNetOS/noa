/**
 * Graph Widget
 * 
 * Interactive network/graph visualization for displaying relationships,
 * dependencies, or any graph-based data structures.
 * 
 * For MVP, uses simple SVG rendering. In production, consider:
 * - D3.js for complex visualizations
 * - Cytoscape.js for large graphs
 * - vis.js for interactive network graphs
 * 
 * Rust Translation (Dioxus):
 * Use egui for rendering or plotters for static graphs
 */

'use client';

import React, { useState, useRef } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ZoomIn, ZoomOut, Maximize2 } from 'lucide-react';
import { motion } from 'framer-motion';

export interface GraphNode {
  id: string;
  label: string;
  x?: number;
  y?: number;
  color?: string;
}

export interface GraphEdge {
  from: string;
  to: string;
  label?: string;
  weight?: number;
}

export interface GraphProps {
  title?: string;
  description?: string;
  nodes: GraphNode[];
  edges: GraphEdge[];
  width?: number;
  height?: number;
  className?: string;
}

export function Graph({
  title = 'Graph Visualization',
  description,
  nodes,
  edges,
  width = 600,
  height = 400,
  className = '',
}: GraphProps) {
  const [zoom, setZoom] = useState(1);
  const [panX, setPanX] = useState(0);
  const [panY, setPanY] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const svgRef = useRef<SVGSVGElement>(null);

  // Auto-layout nodes if positions not provided
  const layoutNodes = nodes.map((node, i) => {
    if (node.x !== undefined && node.y !== undefined) {
      return node;
    }
    // Simple circle layout
    const angle = (2 * Math.PI * i) / nodes.length;
    const radius = Math.min(width, height) * 0.35;
    return {
      ...node,
      x: width / 2 + radius * Math.cos(angle),
      y: height / 2 + radius * Math.sin(angle),
    };
  });

  const handleMouseDown = (e: React.MouseEvent) => {
    setIsDragging(true);
    setDragStart({ x: e.clientX - panX, y: e.clientY - panY });
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    if (isDragging) {
      setPanX(e.clientX - dragStart.x);
      setPanY(e.clientY - dragStart.y);
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
  };

  const handleZoomIn = () => setZoom(Math.min(zoom * 1.2, 5));
  const handleZoomOut = () => setZoom(Math.max(zoom / 1.2, 0.2));
  const handleReset = () => {
    setZoom(1);
    setPanX(0);
    setPanY(0);
  };

  return (
    <Card className={className}>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div>
            <CardTitle>{title}</CardTitle>
            {description && <CardDescription>{description}</CardDescription>}
          </div>
          
          <div className="flex items-center gap-2">
            <Button variant="outline" size="sm" onClick={handleZoomOut}>
              <ZoomOut className="w-4 h-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={handleZoomIn}>
              <ZoomIn className="w-4 h-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={handleReset}>
              <Maximize2 className="w-4 h-4" />
            </Button>
          </div>
        </div>
      </CardHeader>
      
      <CardContent>
        <div 
          className="relative overflow-hidden border rounded-lg bg-muted/20"
          style={{ width, height }}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
        >
          <svg
            ref={svgRef}
            width={width}
            height={height}
            className="cursor-move"
          >
            <g transform={`translate(${panX}, ${panY}) scale(${zoom})`}>
              {/* Draw edges */}
              {edges.map((edge, i) => {
                const fromNode = layoutNodes.find(n => n.id === edge.from);
                const toNode = layoutNodes.find(n => n.id === edge.to);
                
                if (!fromNode || !toNode) return null;
                
                return (
                  <g key={i}>
                    <motion.line
                      x1={fromNode.x}
                      y1={fromNode.y}
                      x2={toNode.x}
                      y2={toNode.y}
                      stroke="currentColor"
                      strokeWidth={edge.weight || 2}
                      opacity={0.6}
                      initial={{ pathLength: 0 }}
                      animate={{ pathLength: 1 }}
                      transition={{ duration: 0.5, delay: i * 0.05 }}
                    />
                    
                    {edge.label && (
                      <text
                        x={(fromNode.x! + toNode.x!) / 2}
                        y={(fromNode.y! + toNode.y!) / 2}
                        fontSize="10"
                        fill="currentColor"
                        textAnchor="middle"
                        opacity={0.8}
                      >
                        {edge.label}
                      </text>
                    )}
                  </g>
                );
              })}
              
              {/* Draw nodes */}
              {layoutNodes.map((node, i) => (
                <g key={node.id}>
                  <motion.circle
                    cx={node.x}
                    cy={node.y}
                    r={20}
                    fill={node.color || 'hsl(var(--primary))'}
                    stroke="currentColor"
                    strokeWidth={2}
                    className="cursor-pointer"
                    initial={{ scale: 0, opacity: 0 }}
                    animate={{ scale: 1, opacity: 1 }}
                    transition={{ duration: 0.3, delay: i * 0.05 }}
                    whileHover={{ scale: 1.2 }}
                  />
                  
                  <text
                    x={node.x}
                    y={node.y}
                    fontSize="12"
                    fill="white"
                    textAnchor="middle"
                    dominantBaseline="middle"
                    pointerEvents="none"
                  >
                    {node.label}
                  </text>
                </g>
              ))}
            </g>
          </svg>
          
          <div className="absolute bottom-2 right-2 text-xs text-muted-foreground bg-background/80 px-2 py-1 rounded">
            Zoom: {(zoom * 100).toFixed(0)}%
          </div>
        </div>
        
        <div className="mt-2 text-xs text-muted-foreground">
          {nodes.length} nodes, {edges.length} edges
        </div>
      </CardContent>
    </Card>
  );
}

/**
 * Rust/Dioxus Translation:
 * 
 * ```rust
 * use plotters::prelude::*;
 * 
 * #[component]
 * pub fn Graph(cx: Scope, nodes: Vec<Node>, edges: Vec<Edge>) -> Element {
 *     let zoom = use_state(cx, || 1.0);
 *     let pan = use_state(cx, || (0, 0));
 *     
 *     cx.render(rsx! {
 *         div { class: "graph-widget",
 *             // Use plotters or egui for rendering
 *             canvas {
 *                 // Render graph with Rust graphics library
 *             }
 *             
 *             div { class: "controls",
 *                 button { onclick: |_| zoom.set(zoom.get() * 1.2), "Zoom In" }
 *                 button { onclick: |_| zoom.set(zoom.get() / 1.2), "Zoom Out" }
 *             }
 *         }
 *     })
 * }
 * ```
 */