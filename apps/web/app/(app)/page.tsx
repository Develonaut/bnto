import { AppShellContent, Divider } from "@bnto/ui";
import { HeroSidebar } from "@/components/blocks/HeroSidebar";
import { RecipeMarquee } from "@/components/blocks/RecipeMarquee";

import { HowItWorksSection } from "./_components/HowItWorksSection";
import { NoCatchSection } from "./_components/NoCatchSection";

/* ── Home page ───────────────────────────────────────────────── */

export default function Home() {
  return (
    <AppShellContent>
      {/* Hero — sidebar + recipe marquee */}
      <div className="grid items-start gap-12 lg:grid-cols-[2fr_3fr] lg:gap-20">
        <HeroSidebar showCta={false} />
        <RecipeMarquee />
      </div>

      <Divider label="Free. No signup." />
      <HowItWorksSection />

      <Divider label="Open source." />
      <NoCatchSection />
    </AppShellContent>
  );
}
