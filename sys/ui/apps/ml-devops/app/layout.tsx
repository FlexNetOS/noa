import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { Providers } from './providers'
import { UmamiAnalytics } from '@/components/analytics/umami-analytics'

const inter = Inter({ subsets: ['latin'] })

export const dynamic = 'force-dynamic'

export const metadata: Metadata = {
  metadataBase: new URL(process.env.NEXTAUTH_URL || 'http://localhost:3000'),
  title: 'ML DevOps Platform - Event-Driven Architecture',
  description: 'Event-sourced ML DevOps platform with streaming chat, widget system, and event replay capabilities',
  icons: {
    icon: '/favicon.svg',
    shortcut: '/favicon.svg',
  },
  openGraph: {
    title: 'ML DevOps Platform',
    description: 'Event-driven ML DevOps platform for modern workflows',
    images: ['/og-image.png'],
  },
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <script src="https://apps.abacus.ai/chatllm/appllm-lib.js"></script>
      </head>
      <body className={inter.className}>
        <Providers>
          {children}
        </Providers>
        <UmamiAnalytics />
      </body>
    </html>
  )
}
