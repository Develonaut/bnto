import type { Metadata } from "next";
import { Geist, Inter } from "next/font/google";
import { ConvexAuthNextjsServerProvider } from "@bnto/auth/server";
import { Navbar } from "@/components/blocks/Navbar";
import { ThemeProvider } from "@/components/ThemeProvider";
import { Providers } from "./providers";
import "./globals.css";

const geist = Geist({
  subsets: ["latin"],
  variable: "--font-geist",
  display: "swap",
});

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
});

export const metadata: Metadata = {
  title: {
    default: "bnto — Compress, Clean & Convert Files Free",
    template: "%s — bnto",
  },
  description:
    "Compress images, clean CSVs, rename files, and convert formats — free, instant, 100% in your browser. No signup, no upload. Open source.",
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
  authors: [{ name: "bnto" }],
  creator: "bnto",
  publisher: "bnto",
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
    title: "bnto — Compress, Clean & Convert Files Free",
    description:
      "Compress images, clean CSVs, rename files, and convert formats — free, instant, 100% in your browser. No signup, no upload. Open source.",
    siteName: "bnto",
  },
  twitter: {
    card: "summary_large_image",
    title: "bnto — Compress, Clean & Convert Files Free",
    description:
      "Compress images, clean CSVs, rename files, and convert formats — free, instant, 100% in your browser. No signup, no upload. Open source.",
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
        <body className={`${geist.variable} ${inter.variable} antialiased`}>
          <ThemeProvider
            attribute="class"
            defaultTheme="light"
            themes={["light", "sunset", "dark"]}
            enableSystem
            disableTransitionOnChange
          >
            <Providers>
              <Navbar />
              {children}
            </Providers>
          </ThemeProvider>
        </body>
      </html>
    </ConvexAuthNextjsServerProvider>
  );
}
