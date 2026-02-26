import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Privacy Policy",
  description:
    "bnto privacy policy. Your files are processed in your browser and never uploaded to our servers. No tracking, no advertising, minimal data collection.",
};

export default function PrivacyLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return children;
}
