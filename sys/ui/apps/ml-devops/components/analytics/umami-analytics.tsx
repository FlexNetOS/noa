/**
 * Umami Analytics Component
 * 
 * Privacy-focused, GDPR-compliant analytics integration.
 * - No cookies, no PII collection
 * - Self-hosted or cloud options
 * - Custom event tracking for workflows and widgets
 * 
 * Rust/Tauri Note:
 * - Can be disabled in desktop mode or use local analytics
 * - Event tracking can be routed through Tauri commands
 */

'use client';

import Script from 'next/script';
import { useEffect } from 'react';
import { getconfigs } from '@/lib/configs';

// Extend window object for Umami
declare global
{
  interface Window
  {
    umami?: {
      track: ( eventName: string, eventData?: Record<string, string | number | boolean> ) => void;
    };
  }
}

export function UmamiAnalytics ()
{
  const configs = getconfigs();
  const websiteId = process.env.NEXT_PUBLIC_UMAMI_WEBSITE_ID;
  const host = process.env.NEXT_PUBLIC_UMAMI_HOST || 'https://cloud.umami.is';
  const scriptPath = process.env.NEXT_PUBLIC_UMAMI_SCRIPT_PATH || '/script.js';
  const disableLocalhost = process.env.NEXT_PUBLIC_UMAMI_DISABLE_LOCALHOST_TRACKING !== 'false';

  useEffect( () =>
  {
    // Disable tracking on localhost if configsured
    if ( disableLocalhost && typeof window !== 'undefined' )
    {
      const isLocalhost = window.location.hostname === 'localhost' ||
        window.location.hostname === '127.0.0.1' ||
        window.location.hostname === '';
      if ( isLocalhost )
      {
        console.log( '[Umami] Analytics disabled on localhost' );
        return;
      }
    }
  }, [ disableLocalhost ] );

  // Don't render if no website ID is configsured
  if ( !websiteId )
  {
    // Only log in development mode to avoid console noise in production
    if ( process.env.NODE_ENV === 'development' )
    {
      console.log( '[Umami] Website ID not configsured. Analytics disabled.' );
    }
    return null;
  }

  const scriptUrl = `${ host }${ scriptPath }`;

  return (
    <Script
      async
      src={ scriptUrl }
      data-website-id={ websiteId }
      data-domains={ process.env.NEXT_PUBLIC_UMAMI_DOMAINS }
      strategy="afterInteractive"
      onLoad={ () =>
      {
        console.log( '[Umami] Analytics loaded successfully' );
      } }
      onError={ ( e ) =>
      {
        console.error( '[Umami] Failed to load analytics:', e );
      } }
    />
  );
}

/**
 * Hook for tracking custom events
 * 
 * Usage:
 *   const trackEvent = useUmamiTracking();
 *   trackEvent('workflow-completed', { workflowId: '123', duration: 45 });
 */
export function useUmamiTracking ()
{
  return ( eventName: string, eventData?: Record<string, string | number | boolean> ) =>
  {
    if ( typeof window !== 'undefined' && window.umami )
    {
      try
      {
        window.umami.track( eventName, eventData );
      } catch ( error )
      {
        console.error( '[Umami] Failed to track event:', error );
      }
    }
  };
}

/**
 * Helper function to track events from anywhere in the app
 * Can be used without React hooks
 */
export function trackUmamiEvent (
  eventName: string,
  eventData?: Record<string, string | number | boolean>
)
{
  if ( typeof window !== 'undefined' && window.umami )
  {
    try
    {
      window.umami.track( eventName, eventData );
    } catch ( error )
    {
      console.error( '[Umami] Failed to track event:', error );
    }
  }
}
