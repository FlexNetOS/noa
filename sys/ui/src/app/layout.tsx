import type { Metadata } from "next";
import "./globals.css";
import "../styles/focus.css";
import "../styles/rtl.css";
import AccessibilityProvider from "./AccessibilityProvider";

export const metadata: Metadata = {
    title: "NOA - Autonomous AI Assistant",
    description: "Your local-first, privacy-preserving AI assistant",
};

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="en">
            <body className="antialiased">
                <AccessibilityProvider>{children}</AccessibilityProvider>
            </body>
        </html>
    );
}
