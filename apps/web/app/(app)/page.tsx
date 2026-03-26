import { AppShellContent, Divider } from "@bnto/ui";

import { HeroBentoGrid } from "./_components/HeroBentoGrid";
import { HowItWorksSection } from "./_components/HowItWorksSection";
import { NoCatchSection } from "./_components/NoCatchSection";

/* ── Home page ───────────────────────────────────────────────── */

export default function Home() {
  return (
    <AppShellContent>
      <HeroBentoGrid />

      <Divider label="Free. No signup." />
      <HowItWorksSection />

      <Divider label="Open source." />
      <NoCatchSection />
    </AppShellContent>
  );
}
