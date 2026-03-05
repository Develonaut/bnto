import type { Metadata } from "next";

import { Pricing } from "@/components/blocks/Pricing";
import { AppShellContent } from "@bnto/ui";

export const metadata: Metadata = {
  title: "Pricing",
  description: "All browser tools free, unlimited, forever. No signup required.",
};

export default function PricingPage() {
  return (
    <AppShellContent>
      <Pricing />
    </AppShellContent>
  );
}
