import { AppShellContent, Divider, Stack } from "@bnto/ui";
import { HeroInstall } from "@/components/blocks/HeroInstall";
import { HeroSidebar } from "@/components/blocks/HeroSidebar";
import { HeroTerminal } from "@/components/blocks/HeroTerminal";

import { HouseSpecialsSection } from "./_components/HouseSpecialsSection";
import { NoCatchSection } from "./_components/NoCatchSection";
import { WhatsInTheBoxSection } from "./_components/WhatsInTheBoxSection";

/* ── Home page ───────────────────────────────────────────────── */

export default function Home() {
  return (
    <AppShellContent>
      {/* Hero — sidebar + install + terminal demo */}
      <div className="grid items-center gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <HeroSidebar />
        <Stack className="gap-4">
          <HeroInstall />
          <HeroTerminal />
        </Stack>
      </div>

      <Divider label="What's in the box" />
      <WhatsInTheBoxSection />

      <Divider label="House specials" />
      <HouseSpecialsSection />

      <Divider label="Open kitchen" />
      <NoCatchSection />
    </AppShellContent>
  );
}
