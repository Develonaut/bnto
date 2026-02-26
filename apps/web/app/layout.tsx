import type { Metadata } from "next";
import { Geist, Geist_Mono, Inter } from "next/font/google";
import { ConvexAuthNextjsServerProvider } from "@bnto/auth/server";
import { AppShell } from "@/components/ui/AppShell";
import { ThemeProvider } from "@/components/ThemeProvider";
import {
  DEFAULT_TITLE,
  SITE_DESCRIPTION,
  SITE_TITLE,
  TITLE_TEMPLATE,
} from "@/lib/copy";
import { Providers } from "./providers";
import "./globals.css";

const geist = Geist({
  subsets: ["latin"],
  variable: "--font-display",
  display: "swap",
});

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-sans",
});

const geistMono = Geist_Mono({
  subsets: ["latin"],
  variable: "--font-mono",
});

export const metadata: Metadata = {
  title: {
    default: DEFAULT_TITLE,
    template: TITLE_TEMPLATE,
  },
  description: SITE_DESCRIPTION,
  keywords: [
    "compress images free",
    "clean csv online",
    "rename files online",
    "convert image format",
    "resize images free",
    "rename csv columns",
    "free online tools",
    "browser-based tools",
    "no upload file tools",
    "batch file processing",
  ],
  authors: [{ name: SITE_TITLE }],
  creator: SITE_TITLE,
  publisher: SITE_TITLE,
  robots: {
    index: true,
    follow: true,
  },
  icons: {
    icon: [
      { url: "/favicon/favicon.ico", sizes: "48x48" },
      { url: "/favicon/favicon.svg", type: "image/svg+xml" },
      { url: "/favicon/favicon-96x96.png", sizes: "96x96", type: "image/png" },
    ],
    apple: [{ url: "/favicon/apple-touch-icon.png", sizes: "180x180" }],
    shortcut: [{ url: "/favicon/favicon.ico" }],
  },
  openGraph: {
    title: DEFAULT_TITLE,
    description: SITE_DESCRIPTION,
    siteName: SITE_TITLE,
  },
  twitter: {
    card: "summary_large_image",
    title: DEFAULT_TITLE,
    description: SITE_DESCRIPTION,
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ConvexAuthNextjsServerProvider>
      <html lang="en" suppressHydrationWarning>
        <head />
        <body className={`${geist.variable} ${inter.variable} ${geistMono.variable} antialiased`}>
          <ThemeProvider
            attribute="class"
            defaultTheme="light"
            themes={["light", "sunset", "dark"]}
            value={{ light: "light", sunset: "munich", dark: "tokyo" }}
            enableSystem
            disableTransitionOnChange
          >
            <Providers>
              <AppShell>
                {children}
              </AppShell>
            </Providers>
          </ThemeProvider>
        </body>
      </html>
    </ConvexAuthNextjsServerProvider>
  );
}
