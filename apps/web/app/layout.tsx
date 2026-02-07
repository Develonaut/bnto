import type { Metadata } from "next";
import { Providers } from "./providers";
import { Nav } from "./nav";
import "./globals.css";

export const metadata: Metadata = {
  title: "Bnto",
  description: "Workflow automation, simplified.",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <Providers>
          <Nav />
          {children}
        </Providers>
      </body>
    </html>
  );
}
