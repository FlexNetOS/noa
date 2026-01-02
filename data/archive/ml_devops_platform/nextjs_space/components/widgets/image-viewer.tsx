/**
 * ImageViewer Widget
 * 
 * Image display with zoom, pan, and rotation controls.
 * Designed for viewing ML training images, dataset samples,
 * or any image-based results.
 * 
 * Rust Translation (Dioxus):
 * Use image crate for loading and egui for display
 */

'use client';

import React, { useState } from 'react';
import Image from 'next/image';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { ZoomIn, ZoomOut, RotateCw, Maximize2, Download } from 'lucide-react';
import { motion } from 'framer-motion';

export interface ImageViewerProps {
  title?: string;
  description?: string;
  src: string;
  alt?: string;
  width?: number;
  height?: number;
  downloadable?: boolean;
  className?: string;
}

export function ImageViewer({
  title = 'Image Viewer',
  description,
  src,
  alt = 'Image',
  width = 600,
  height = 400,
  downloadable = true,
  className = '',
}: ImageViewerProps) {
  const [zoom, setZoom] = useState(1);
  const [rotation, setRotation] = useState(0);
  const [panX, setPanX] = useState(0);
  const [panY, setPanY] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });

  const handleMouseDown = (e: React.MouseEvent) => {
    if (zoom > 1) {
      setIsDragging(true);
      setDragStart({ x: e.clientX - panX, y: e.clientY - panY });
    }
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
  const handleZoomOut = () => {
    const newZoom = Math.max(zoom / 1.2, 1);
    setZoom(newZoom);
    if (newZoom === 1) {
      setPanX(0);
      setPanY(0);
    }
  };
  
  const handleRotate = () => setRotation((rotation + 90) % 360);
  
  const handleReset = () => {
    setZoom(1);
    setRotation(0);
    setPanX(0);
    setPanY(0);
  };

  const handleDownload = async () => {
    try {
      const response = await fetch(src);
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = alt || 'image';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Failed to download image:', error);
    }
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
            <Button variant="outline" size="sm" onClick={handleRotate}>
              <RotateCw className="w-4 h-4" />
            </Button>
            <Button variant="outline" size="sm" onClick={handleReset}>
              <Maximize2 className="w-4 h-4" />
            </Button>
            {downloadable && (
              <Button variant="outline" size="sm" onClick={handleDownload}>
                <Download className="w-4 h-4" />
              </Button>
            )}
          </div>
        </div>
      </CardHeader>
      
      <CardContent>
        <div 
          className="relative overflow-hidden border rounded-lg bg-muted/20 flex items-center justify-center"
          style={{ width, height }}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
        >
          <motion.div
            animate={{
              scale: zoom,
              rotate: rotation,
              x: panX,
              y: panY,
            }}
            transition={{ type: 'spring', stiffness: 300, damping: 30 }}
            style={{
              cursor: zoom > 1 ? 'move' : 'default',
            }}
          >
            <div className="relative" style={{ width: width * 0.8, height: height * 0.8 }}>
              <Image
                src={src}
                alt={alt}
                fill
                className="object-contain"
                draggable={false}
              />
            </div>
          </motion.div>
          
          <div className="absolute bottom-2 right-2 text-xs text-muted-foreground bg-background/80 px-2 py-1 rounded">
            Zoom: {(zoom * 100).toFixed(0)}% | Rotation: {rotation}°
          </div>
        </div>
        
        {zoom > 1 && (
          <div className="mt-2 text-xs text-muted-foreground text-center">
            Drag to pan the image
          </div>
        )}
      </CardContent>
    </Card>
  );
}

/**
 * Rust/Dioxus Translation:
 * 
 * ```rust
 * use image::DynamicImage;
 * 
 * #[component]
 * pub fn ImageViewer(
 *     cx: Scope,
 *     title: String,
 *     src: String,
 *     alt: String,
 * ) -> Element {
 *     let zoom = use_state(cx, || 1.0);
 *     let rotation = use_state(cx, || 0);
 *     let pan = use_state(cx, || (0, 0));
 *     
 *     // Load image using image crate
 *     let img = use_future(cx, (), |_| async move {
 *         image::open(&src).ok()
 *     });
 *     
 *     cx.render(rsx! {
 *         div { class: "image-viewer",
 *             div { class: "controls",
 *                 button { onclick: |_| zoom.modify(|z| z * 1.2), "Zoom In" }
 *                 button { onclick: |_| zoom.modify(|z| z / 1.2), "Zoom Out" }
 *                 button { onclick: |_| rotation.modify(|r| (r + 90) % 360), "Rotate" }
 *             }
 *             
 *             div { 
 *                 class: "image-container",
 *                 style: "transform: scale({zoom}) rotate({rotation}deg) translate({pan.0}px, {pan.1}px)",
 *                 
 *                 img {
 *                     src: "{src}",
 *                     alt: "{alt}",
 *                     draggable: false,
 *                 }
 *             }
 *         }
 *     })
 * }
 * ```
 */