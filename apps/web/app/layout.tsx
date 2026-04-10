import type { ReactNode } from "react";
import type { Metadata } from "next";
import { Geist, Geist_Mono, Inter } from "next/font/google";
import { ConvexAuthNextjsServerProvider } from "@bnto/auth/server";
import { AppShell } from "@bnto/ui";
import { ThemeProvider } from "@/components/ThemeProvider";
import { t } from "@bnto/i18n";
import { BASE_URL } from "@/lib/constants";
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
  metadataBase: new URL(BASE_URL),
  title: {
    default: t("site.defaultTitle"),
    template: t("site.titleTemplate"),
  },
  description: t("site.description"),
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
    "visual workflow editor",
    "custom recipes",
    "file tools no signup",
    "privacy first tools",
    "rust webassembly tools",
  ],
  authors: [{ name: t("site.title") }],
  creator: t("site.title"),
  publisher: t("site.title"),
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
    title: t("site.defaultTitle"),
    description: t("site.description"),
    siteName: t("site.title"),
  },
  twitter: {
    card: "summary_large_image",
    title: t("site.defaultTitle"),
    description: t("site.description"),
  },
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <ConvexAuthNextjsServerProvider>
      <html lang="en" suppressHydrationWarning>
        <head />
        <body className={`${geist.variable} ${inter.variable} ${geistMono.variable} antialiased`}>
          <ThemeProvider
            attribute="class"
            defaultTheme="light"
            themes={["light", "sunset", "dark"]}
            value={{ light: "light", sunset: "monaco", dark: "tokyo" }}
            enableSystem
            disableTransitionOnChange
          >
            <Providers>
              <AppShell>{children}</AppShell>
            </Providers>
          </ThemeProvider>
        </body>
      </html>
    </ConvexAuthNextjsServerProvider>
  );
}
