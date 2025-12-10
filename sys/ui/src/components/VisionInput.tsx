'use client';

import { useState, useRef, useEffect } from 'react';
import { Camera, Monitor, X, Check } from 'lucide-react';
import { hardwareCapabilities } from '@/services/hardwareCapabilities';
import { cn } from '@/lib/utils';

interface VisionInputProps {
  onCapture: (data: Blob | File) => void;
  onCancel?: () => void;
  disabled?: boolean;
}

type CaptureMode = 'camera' | 'screen' | null;

/**
 * Vision Input Component
 *
 * Provides vision input using camera or screen capture with graceful degradation.
 */
export default function VisionInput({ onCapture, onCancel, disabled }: VisionInputProps) {
  const [isAvailable, setIsAvailable] = useState(false);
  const [captureMode, setCaptureMode] = useState<CaptureMode>(null);
  const [stream, setStream] = useState<MediaStream | null>(null);
  const [capturedImage, setCapturedImage] = useState<string | null>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const checkAvailability = async () => {
      const capabilities = await hardwareCapabilities.detect();
      setIsAvailable(capabilities.vision.available);
    };

    checkAvailability();

    return () => {
      // Cleanup stream on unmount
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
    };
  }, [stream]);

  const startCamera = async () => {
    try {
      const mediaStream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: 'environment' },
      });

      setStream(mediaStream);
      setCaptureMode('camera');

      if (videoRef.current) {
        videoRef.current.srcObject = mediaStream;
      }
    } catch (error) {
      console.error('Failed to start camera:', error);
      alert('Failed to access camera. Please check permissions.');
    }
  };

  const startScreenCapture = async () => {
    try {
      const mediaStream = await navigator.mediaDevices.getDisplayMedia({
        video: true,
      });

      setStream(mediaStream);
      setCaptureMode('screen');

      if (videoRef.current) {
        videoRef.current.srcObject = mediaStream;
      }
    } catch (error) {
      console.error('Failed to start screen capture:', error);
      alert('Failed to capture screen. Please check permissions.');
    }
  };

  const captureImage = () => {
    if (!videoRef.current || !canvasRef.current) return;

    const video = videoRef.current;
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');

    if (!ctx) return;

    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    ctx.drawImage(video, 0, 0);

    canvas.toBlob((blob) => {
      if (blob) {
        const imageUrl = URL.createObjectURL(blob);
        setCapturedImage(imageUrl);
        onCapture(blob);
      }
    }, 'image/png');
  };

  const stopCapture = () => {
    if (stream) {
      stream.getTracks().forEach(track => track.stop());
      setStream(null);
    }
    setCaptureMode(null);
    setCapturedImage(null);
    if (videoRef.current) {
      videoRef.current.srcObject = null;
    }
  };

  const handleCancel = () => {
    stopCapture();
    onCancel?.();
  };

  if (!isAvailable) {
    return (
      <div className="text-sm text-slate-400 p-4 bg-slate-800/50 rounded-lg">
        Vision input not available on this device. Camera or screen capture permissions may be required.
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {!captureMode ? (
        <div className="flex gap-2">
          <button
            onClick={startCamera}
            disabled={disabled}
            aria-label="Start camera capture"
            className="flex items-center gap-2 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Camera className="w-4 h-4" />
            Camera
          </button>
          <button
            onClick={startScreenCapture}
            disabled={disabled}
            aria-label="Start screen capture"
            className="flex items-center gap-2 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <Monitor className="w-4 h-4" />
            Screen
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="relative">
            <video
              ref={videoRef}
              autoPlay
              playsInline
              className={cn(
                'w-full rounded-lg',
                capturedImage && 'hidden'
              )}
            />
            {capturedImage && (
              // eslint-disable-next-line @next/next/no-img-element
              <img
                src={capturedImage}
                alt="Captured"
                className="w-full rounded-lg"
              />
            )}
            <canvas ref={canvasRef} className="hidden" />
          </div>

          <div className="flex gap-2">
            {!capturedImage ? (
              <>
                <button
                  onClick={captureImage}
                  aria-label="Capture current frame"
                  className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
                >
                  <Check className="w-4 h-4" />
                  Capture
                </button>
                <button
                  onClick={stopCapture}
                  aria-label="Cancel capture"
                  className="flex items-center gap-2 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 rounded-lg transition-colors"
                >
                  <X className="w-4 h-4" />
                  Cancel
                </button>
              </>
            ) : (
              <>
                <button
                  onClick={handleCancel}
                  aria-label="Confirm captured image"
                  className="flex items-center gap-2 px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg transition-colors"
                >
                  <Check className="w-4 h-4" />
                  Done
                </button>
                <button
                  onClick={() => {
                    setCapturedImage(null);
                    stopCapture();
                  }}
                  aria-label="Retake image"
                  className="flex items-center gap-2 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 rounded-lg transition-colors"
                >
                  <X className="w-4 h-4" />
                  Retake
                </button>
              </>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
