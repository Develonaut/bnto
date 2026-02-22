import type { Metadata } from "next";
import { DM_Sans, Inter } from "next/font/google";
import { getToken } from "@bnto/auth/server";
import { ThemeProvider } from "@bnto/ui/theme-provider";
import { Providers } from "./providers";
import "@bnto/ui/globals.css";

const dmSans = DM_Sans({
  subsets: ["latin"],
  variable: "--font-dm-sans",
});

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
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
        className={`${dmSans.variable} ${inter.variable} antialiased`}
      >
        <ThemeProvider
          attribute="class"
          defaultTheme="light"
          enableSystem
          disableTransitionOnChange
        >
          <Providers initialToken={initialToken}>
            {children}
          </Providers>
        </ThemeProvider>
      </body>
    </html>
  );
}
