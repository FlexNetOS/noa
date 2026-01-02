/**
 * VideoPlayer Widget
 * 
 * Video playback with standard controls.
 * Designed for viewing training progress videos, demos,
 * or any video-based ML outputs.
 * 
 * Rust Translation (Dioxus):
 * Use gstreamer bindings or platform-specific video APIs
 */

'use client';

import React, { useRef, useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Slider } from '@/components/ui/slider';
import { 
  Play, 
  Pause, 
  Volume2, 
  VolumeX, 
  Maximize, 
  SkipBack, 
  SkipForward 
} from 'lucide-react';
import { motion } from 'framer-motion';

export interface VideoPlayerProps {
  title?: string;
  description?: string;
  src: string;
  poster?: string;
  width?: number;
  height?: number;
  autoPlay?: boolean;
  loop?: boolean;
  className?: string;
}

export function VideoPlayer({
  title = 'Video Player',
  description,
  src,
  poster,
  width = 640,
  height = 360,
  autoPlay = false,
  loop = false,
  className = '',
}: VideoPlayerProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [volume, setVolume] = useState(1);
  const [isMuted, setIsMuted] = useState(false);
  const [showControls, setShowControls] = useState(true);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return;

    const updateTime = () => setCurrentTime(video.currentTime);
    const updateDuration = () => setDuration(video.duration);
    const handleEnded = () => setIsPlaying(false);

    video.addEventListener('timeupdate', updateTime);
    video.addEventListener('loadedmetadata', updateDuration);
    video.addEventListener('ended', handleEnded);

    return () => {
      video.removeEventListener('timeupdate', updateTime);
      video.removeEventListener('loadedmetadata', updateDuration);
      video.removeEventListener('ended', handleEnded);
    };
  }, []);

  const togglePlay = () => {
    const video = videoRef.current;
    if (!video) return;

    if (isPlaying) {
      video.pause();
    } else {
      video.play();
    }
    setIsPlaying(!isPlaying);
  };

  const handleSeek = (value: number[]) => {
    const video = videoRef.current;
    if (!video) return;

    video.currentTime = value[0];
    setCurrentTime(value[0]);
  };

  const handleVolumeChange = (value: number[]) => {
    const video = videoRef.current;
    if (!video) return;

    const newVolume = value[0];
    video.volume = newVolume;
    setVolume(newVolume);
    setIsMuted(newVolume === 0);
  };

  const toggleMute = () => {
    const video = videoRef.current;
    if (!video) return;

    if (isMuted) {
      video.volume = volume || 0.5;
      setIsMuted(false);
    } else {
      video.volume = 0;
      setIsMuted(true);
    }
  };

  const skip = (seconds: number) => {
    const video = videoRef.current;
    if (!video) return;

    video.currentTime = Math.max(0, Math.min(duration, video.currentTime + seconds));
  };

  const toggleFullscreen = () => {
    const video = videoRef.current;
    if (!video) return;

    if (document.fullscreenElement) {
      document.exitFullscreen();
    } else {
      video.requestFullscreen();
    }
  };

  const formatTime = (time: number) => {
    const minutes = Math.floor(time / 60);
    const seconds = Math.floor(time % 60);
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      
      <CardContent>
        <div 
          className="relative rounded-lg overflow-hidden bg-black"
          style={{ width, height }}
          onMouseEnter={() => setShowControls(true)}
          onMouseLeave={() => setShowControls(isPlaying ? false : true)}
        >
          <video
            ref={videoRef}
            src={src}
            poster={poster}
            width={width}
            height={height}
            autoPlay={autoPlay}
            loop={loop}
            className="w-full h-full object-contain"
            onClick={togglePlay}
          />
          
          <motion.div
            className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/80 to-transparent p-4"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: showControls ? 1 : 0, y: showControls ? 0 : 20 }}
            transition={{ duration: 0.2 }}
          >
            {/* Progress bar */}
            <Slider
              value={[currentTime]}
              max={duration || 100}
              step={0.1}
              onValueChange={handleSeek}
              className="mb-2"
            />
            
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Button 
                  variant="ghost" 
                  size="sm" 
                  onClick={() => skip(-10)}
                  className="text-white hover:bg-white/20"
                >
                  <SkipBack className="w-4 h-4" />
                </Button>
                
                <Button 
                  variant="ghost" 
                  size="sm" 
                  onClick={togglePlay}
                  className="text-white hover:bg-white/20"
                >
                  {isPlaying ? (
                    <Pause className="w-5 h-5" />
                  ) : (
                    <Play className="w-5 h-5" />
                  )}
                </Button>
                
                <Button 
                  variant="ghost" 
                  size="sm" 
                  onClick={() => skip(10)}
                  className="text-white hover:bg-white/20"
                >
                  <SkipForward className="w-4 h-4" />
                </Button>
                
                <div className="flex items-center gap-2 ml-4">
                  <Button 
                    variant="ghost" 
                    size="sm" 
                    onClick={toggleMute}
                    className="text-white hover:bg-white/20"
                  >
                    {isMuted || volume === 0 ? (
                      <VolumeX className="w-4 h-4" />
                    ) : (
                      <Volume2 className="w-4 h-4" />
                    )}
                  </Button>
                  
                  <Slider
                    value={[isMuted ? 0 : volume]}
                    max={1}
                    step={0.1}
                    onValueChange={handleVolumeChange}
                    className="w-20"
                  />
                </div>
              </div>
              
              <div className="flex items-center gap-2">
                <span className="text-white text-sm">
                  {formatTime(currentTime)} / {formatTime(duration)}
                </span>
                
                <Button 
                  variant="ghost" 
                  size="sm" 
                  onClick={toggleFullscreen}
                  className="text-white hover:bg-white/20"
                >
                  <Maximize className="w-4 h-4" />
                </Button>
              </div>
            </div>
          </motion.div>
        </div>
      </CardContent>
    </Card>
  );
}

/**
 * Rust/Dioxus Translation:
 * 
 * ```rust
 * use gstreamer as gst;
 * 
 * #[component]
 * pub fn VideoPlayer(
 *     cx: Scope,
 *     title: String,
 *     src: String,
 * ) -> Element {
 *     let is_playing = use_state(cx, || false);
 *     let current_time = use_state(cx, || 0.0);
 *     let duration = use_state(cx, || 0.0);
 *     
 *     // Initialize GStreamer pipeline
 *     let pipeline = use_memo(cx, (), |_| {
 *         let pipeline = gst::Pipeline::new(Some("video-player"));
 *         let src = gst::ElementFactory::make("filesrc", Some("source")).unwrap();
 *         // ... setup GStreamer elements
 *         pipeline
 *     });
 *     
 *     cx.render(rsx! {
 *         div { class: "video-player",
 *             div { class: "video-container",
 *                 // Platform-specific video rendering
 *             }
 *             
 *             div { class: "controls",
 *                 button { 
 *                     onclick: move |_| {
 *                         if *is_playing.get() {
 *                             pipeline.set_state(gst::State::Paused);
 *                         } else {
 *                             pipeline.set_state(gst::State::Playing);
 *                         }
 *                         is_playing.modify(|p| !p);
 *                     },
 *                     if *is_playing.get() { "Pause" } else { "Play" }
 *                 }
 *                 
 *                 input {
 *                     r#type: "range",
 *                     min: "0",
 *                     max: "{duration}",
 *                     value: "{current_time}",
 *                     oninput: move |e| {
 *                         let time = e.value.parse::<f64>().unwrap_or(0.0);
 *                         pipeline.seek_simple(
 *                             gst::SeekFlags::FLUSH,
 *                             gst::ClockTime::from_seconds(time as u64)
 *                         );
 *                     }
 *                 }
 *             }
 *         }
 *     })
 * }
 * ```
 */