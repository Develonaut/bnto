import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import { getToken } from "@bnto/auth/server";
import { Providers } from "./providers";
import "@bnto/ui/globals.css";

const fontSans = Geist({
  subsets: ["latin"],
  variable: "--font-sans",
});

const fontMono = Geist_Mono({
  subsets: ["latin"],
  variable: "--font-mono",
});

export const metadata: Metadata = {
  title: {
    default: "bnto -- Free Online Tools",
    template: "%s",
  },
  description:
    "Free online tools to compress images, clean CSVs, rename files, and more. No signup required.",
};

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const initialToken = await getToken();

  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${fontSans.variable} ${fontMono.variable} antialiased`}
      >
        <Providers initialToken={initialToken}>
          {children}
        </Providers>
      </body>
    </html>
  );
}
