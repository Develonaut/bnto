import type { Metadata } from "next";

import { Pricing } from "@/components/blocks/Pricing";
import { AppShell } from "@/components/ui/AppShell";

export const metadata: Metadata = {
  title: "Pricing",
  description: "All browser tools free, unlimited, forever. No signup required.",
};

export default function PricingPage() {
  return (
    <AppShell.Content>
      <Pricing />
    </AppShell.Content>
  );
}
