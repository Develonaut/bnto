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
    default: "Mainline - Modern Next.js Template",
    template: "%s | Mainline",
  },
  description:
    "A modern Next.js template built with shadcn/ui, Tailwind & MDX. Open source - MIT License.",
  keywords: [
    "Next.js",
    "nextjs template",
    "nextjs theme",
    "nextjs starter",
    "shadcn template",
    "shadcn theme",
    "shadcn starter",
    "tailwind template",
    "tailwind theme",
    "tailwind starter",
    "mdx template",
    "mdx theme",
    "mdx starter",
  ],
  authors: [{ name: "shadcnblocks.com" }],
  creator: "shadcnblocks.com",
  publisher: "shadcnblocks.com",
  robots: {
    index: true,
    follow: true,
  },
  icons: {
    icon: [
      { url: "/favicon/favicon.ico", sizes: "48x48" },
      { url: "/favicon/favicon.svg", type: "image/svg+xml" },
      { url: "/favicon/favicon-96x96.png", sizes: "96x96", type: "image/png" },
      { url: "/favicon/favicon.svg", type: "image/svg+xml" },
      { url: "/favicon/favicon.ico" },
    ],
    apple: [{ url: "/favicon/apple-touch-icon.png", sizes: "180x180" }],
    shortcut: [{ url: "/favicon/favicon.ico" }],
  },
  openGraph: {
    title: "Mainline - Modern Next.js Template",
    description:
      "A modern Next.js template built with shadcn/ui, Tailwind & MDX. Open source - MIT License.",
    siteName: "Mainline",
    images: [
      {
        url: "/og-image.jpg",
        width: 1200,
        height: 630,
        alt: "Mainline - Modern Next.js Template",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "Mainline - Modern Next.js Template",
    description:
      "A modern Next.js template built with shadcn/ui, Tailwind & MDX. Open source - MIT License.",
    images: ["/og-image.jpg"],
    creator: "@ausrobdev",
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
