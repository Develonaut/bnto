import { AppShellContent, Divider, Stack } from "@bnto/ui";
import { HeroInstall } from "@/components/blocks/HeroInstall";
import { HeroMascot } from "@/components/blocks/HeroMascot";
import { HeroSidebar } from "@/components/blocks/HeroSidebar";
import { HeroTerminal } from "@/components/blocks/HeroTerminal";

import { BuildYourOwnSection } from "./_components/BuildYourOwnSection";
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
          <div className="relative">
            <HeroTerminal />
            <HeroMascot />
          </div>
        </Stack>
      </div>

      <Divider label="What's in the box" />
      <WhatsInTheBoxSection />

      <Divider label="House specials" />
      <HouseSpecialsSection />

      <Divider label="Build your own" />
      <BuildYourOwnSection />

      <Divider label="Open kitchen" />
      <NoCatchSection />
    </AppShellContent>
  );
}
