'use client';

import React, { useCallback, useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { Upload, X, File, CheckCircle2, AlertCircle } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

/**
 * FileUploader Widget - File upload with drag-and-drop
 * 
 * Features:
 * - Drag and drop interface
 * - Multiple file selection
 * - File type filtering
 * - Upload progress tracking
 * - File preview and removal
 * - Size limit validation
 * 
 * Rust/Dioxus Translation:
 * - Use web_sys for FileReader API
 * - Implement drag/drop events with wasm_bindgen
 * - Use channels for async upload tracking
 * - Map to Dioxus component with hooks
 */

interface UploadedFile {
  id: string;
  name: string;
  size: number;
  type: string;
  progress: number;
  status: 'uploading' | 'completed' | 'error';
  error?: string;
  url?: string;
}

interface FileUploaderConfig {
  maxFiles?: number;
  maxSize?: number; // in bytes
  accept?: string[]; // file types
  uploadEndpoint?: string;
  onUploadComplete?: (files: UploadedFile[]) => void;
}

interface FileUploaderProps {
  config: FileUploaderConfig;
  className?: string;
}

export function FileUploader({ config, className = '' }: FileUploaderProps) {
  const [files, setFiles] = useState<UploadedFile[]>([]);
  const [isDragging, setIsDragging] = useState(false);
  const {
    maxFiles = 10,
    maxSize = 10 * 1024 * 1024, // 10MB default
    accept = [],
    uploadEndpoint = '/api/upload',
    onUploadComplete,
  } = config;

  // Validate file
  const validateFile = (file: File): string | null => {
    if (maxSize && file.size > maxSize) {
      return `File size exceeds ${(maxSize / 1024 / 1024).toFixed(2)}MB`;
    }
    if (accept.length > 0 && !accept.some(type => file.type.match(type))) {
      return `File type not accepted. Allowed: ${accept.join(', ')}`;
    }
    return null;
  };

  // Upload file
  const uploadFile = async (file: File): Promise<UploadedFile> => {
    const fileId = `${Date.now()}-${file.name}`;
    const uploadedFile: UploadedFile = {
      id: fileId,
      name: file.name,
      size: file.size,
      type: file.type,
      progress: 0,
      status: 'uploading',
    };

    // Validate
    const error = validateFile(file);
    if (error) {
      return { ...uploadedFile, status: 'error', error, progress: 0 };
    }

    // Simulate upload (replace with actual API call)
    return new Promise((resolve) => {
      let progress = 0;
      const interval = setInterval(() => {
        progress += 10;
        setFiles(prev =>
          prev.map(f =>
            f.id === fileId ? { ...f, progress: Math.min(progress, 100) } : f
          )
        );

        if (progress >= 100) {
          clearInterval(interval);
          const completed = {
            ...uploadedFile,
            status: 'completed' as const,
            progress: 100,
            url: `/uploads/${file.name}`, // Mock URL
          };
          resolve(completed);
        }
      }, 200);
    });
  };

  // Handle file selection
  const handleFiles = useCallback(
    async (fileList: FileList) => {
      const newFiles = Array.from(fileList);
      
      // Check max files limit
      if (files.length + newFiles.length > maxFiles) {
        alert(`Maximum ${maxFiles} files allowed`);
        return;
      }

      // Create initial file entries
      const initialFiles: UploadedFile[] = newFiles.map((file, idx) => ({
        id: `${Date.now()}-${idx}-${file.name}`,
        name: file.name,
        size: file.size,
        type: file.type,
        progress: 0,
        status: 'uploading' as const,
      }));

      setFiles(prev => [...prev, ...initialFiles]);

      // Upload files
      const uploadPromises = newFiles.map((file, idx) => uploadFile(file));
      const uploadedFiles = await Promise.all(uploadPromises);

      // Update with results
      setFiles(prev => {
        const updated = prev.map(f => {
          const uploaded = uploadedFiles.find(u => u.name === f.name);
          return uploaded || f;
        });
        
        // Notify completion
        if (onUploadComplete) {
          const completed = updated.filter(f => f.status === 'completed');
          onUploadComplete(completed);
        }
        
        return updated;
      });
    },
    [files.length, maxFiles, onUploadComplete]
  );

  // Drag and drop handlers
  const handleDragEnter = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const { files: droppedFiles } = e.dataTransfer;
    if (droppedFiles && droppedFiles.length > 0) {
      handleFiles(droppedFiles);
    }
  };

  // File input handler
  const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      handleFiles(e.target.files);
    }
  };

  // Remove file
  const removeFile = (fileId: string) => {
    setFiles(prev => prev.filter(f => f.id !== fileId));
  };

  // Format file size
  const formatSize = (bytes: number): string => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
    return `${(bytes / 1024 / 1024).toFixed(2)} MB`;
  };

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Upload className="h-5 w-5" />
          File Uploader
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Drop Zone */}
        <div
          className={`
            relative border-2 border-dashed rounded-lg p-8
            transition-colors duration-200
            ${isDragging
              ? 'border-primary bg-primary/10'
              : 'border-muted-foreground/25 hover:border-primary/50'
            }
          `}
          onDragEnter={handleDragEnter}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
        >
          <input
            type="file"
            id="file-upload"
            className="hidden"
            multiple
            accept={accept.join(',')}
            onChange={handleFileInput}
          />
          <label
            htmlFor="file-upload"
            className="flex flex-col items-center justify-center cursor-pointer"
          >
            <Upload className="h-12 w-12 text-muted-foreground mb-4" />
            <p className="text-center text-sm text-muted-foreground mb-2">
              Drag & drop files here, or click to select
            </p>
            <p className="text-xs text-muted-foreground">
              Max {maxFiles} files • Max {(maxSize / 1024 / 1024).toFixed(0)}MB per file
            </p>
            {accept.length > 0 && (
              <p className="text-xs text-muted-foreground mt-1">
                Accepted: {accept.join(', ')}
              </p>
            )}
          </label>
        </div>

        {/* File List */}
        <AnimatePresence>
          {files.length > 0 && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              className="space-y-2"
            >
              {files.map((file) => (
                <motion.div
                  key={file.id}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  exit={{ opacity: 0, x: 20 }}
                  className="flex items-center gap-3 p-3 border rounded-lg bg-card"
                >
                  {/* File Icon */}
                  <div className="flex-shrink-0">
                    {file.status === 'completed' ? (
                      <CheckCircle2 className="h-5 w-5 text-green-500" />
                    ) : file.status === 'error' ? (
                      <AlertCircle className="h-5 w-5 text-destructive" />
                    ) : (
                      <File className="h-5 w-5 text-muted-foreground" />
                    )}
                  </div>

                  {/* File Info */}
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between mb-1">
                      <p className="text-sm font-medium truncate">{file.name}</p>
                      <span className="text-xs text-muted-foreground ml-2">
                        {formatSize(file.size)}
                      </span>
                    </div>

                    {/* Progress */}
                    {file.status === 'uploading' && (
                      <div className="space-y-1">
                        <Progress value={file.progress} className="h-1" />
                        <p className="text-xs text-muted-foreground">
                          {file.progress}% uploaded
                        </p>
                      </div>
                    )}

                    {/* Error */}
                    {file.status === 'error' && file.error && (
                      <p className="text-xs text-destructive">{file.error}</p>
                    )}

                    {/* Success */}
                    {file.status === 'completed' && (
                      <p className="text-xs text-green-600">Upload complete</p>
                    )}
                  </div>

                  {/* Remove Button */}
                  <Button
                    variant="ghost"
                    size="icon"
                    className="flex-shrink-0"
                    onClick={() => removeFile(file.id)}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </motion.div>
              ))}
            </motion.div>
          )}
        </AnimatePresence>

        {/* Summary */}
        {files.length > 0 && (
          <div className="flex items-center justify-between text-sm text-muted-foreground pt-2 border-t">
            <span>
              {files.filter(f => f.status === 'completed').length} / {files.length} completed
            </span>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setFiles([])}
            >
              Clear All
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
