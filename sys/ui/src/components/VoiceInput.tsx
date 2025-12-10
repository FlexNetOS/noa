'use client';

import { useState, useEffect } from 'react';
import { Mic, MicOff } from 'lucide-react';
import { multiModalService } from '@/services/multiModal';

interface VoiceInputProps {
  onTranscript: (text: string) => void;
  disabled?: boolean;
}

/**
 * Voice Input Component
 *
 * Provides voice input using Web Speech API with graceful degradation.
 */
export default function VoiceInput({ onTranscript, disabled }: VoiceInputProps) {
  const [isListening, setIsListening] = useState(false);
  const [isAvailable, setIsAvailable] = useState(false);
  const [recognition, setRecognition] = useState<SpeechRecognition | null>(null);

  useEffect(() => {
    const checkAvailability = async () => {
      const capabilities = await multiModalService.detectCapabilities();
      setIsAvailable(capabilities.voice);
    };

    checkAvailability();

    // Initialize Web Speech API
    if (typeof window !== 'undefined') {
      interface SpeechRecognitionConstructor {
        new (): SpeechRecognition;
      }
      
      const SpeechRecognition = 
        (window as unknown as { webkitSpeechRecognition?: SpeechRecognitionConstructor }).webkitSpeechRecognition ||
        (window as unknown as { SpeechRecognition?: SpeechRecognitionConstructor }).SpeechRecognition;
      
      if (SpeechRecognition) {
        const rec = new SpeechRecognition();
        rec.continuous = false;
        rec.interimResults = false;
        rec.lang = 'en-US';

        rec.onresult = (event: SpeechRecognitionEvent) => {
          const transcript = event.results[0][0].transcript;
          onTranscript(transcript);
          setIsListening(false);
        };

        rec.onerror = (event: SpeechRecognitionErrorEvent) => {
          console.error('Speech recognition error:', event.error);
          setIsListening(false);
        };

        rec.onend = () => {
          setIsListening(false);
        };

        setRecognition(rec);
      }
    }
  }, [onTranscript]);

  const toggleListening = () => {
    if (!recognition || disabled || !isAvailable) return;

    if (isListening) {
      recognition.stop();
      setIsListening(false);
    } else {
      recognition.start();
      setIsListening(true);
    }
  };

  if (!isAvailable) {
    return (
      <div className="text-sm text-slate-400">
        Voice input not available on this device
      </div>
    );
  }

  return (
    <button
      onClick={toggleListening}
      disabled={disabled}
      className={`p-3 rounded-lg transition-colors ${
        isListening
          ? 'bg-red-600 hover:bg-red-700 text-white'
          : 'bg-slate-700 hover:bg-slate-600 text-slate-300'
      } disabled:opacity-50 disabled:cursor-not-allowed`}
      aria-label={isListening ? 'Stop listening' : 'Start listening'}
    >
      {isListening ? (
        <MicOff className="w-5 h-5" />
      ) : (
        <Mic className="w-5 h-5" />
      )}
    </button>
  );
}

