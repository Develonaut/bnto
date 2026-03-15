import {
  PlusIcon,
  RecipeCard,
  RecipeCardContent,
  RecipeCardDescription,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardTitle,
  RecipeCardTags,
} from "@bnto/ui";

import { getBntoIcon } from "@/lib/bntoIcons";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

import { Marquee } from "./Marquee";

/* ── Data — split into 2 columns ──────────────────────────────── */

const half = Math.ceil(BNTO_REGISTRY.length / 2);
const col1 = BNTO_REGISTRY.slice(0, half);
const col2 = BNTO_REGISTRY.slice(half);

/* ── Recipe marquee ────────────────────────────────────────────── */

export function RecipeMarquee() {
  return (
    <div className="relative flex h-[500px] w-full flex-row items-center justify-center gap-4 overflow-hidden">
      <Marquee pauseOnHover vertical className="[--duration:22s]">
        {col1.map((entry) => (
          <MarqueeRecipeCard key={entry.slug} slug={entry.slug} />
        ))}
      </Marquee>
      <Marquee reverse pauseOnHover vertical className="[--duration:28s]">
        {col2.map((entry) => (
          <MarqueeRecipeCard key={entry.slug} slug={entry.slug} />
        ))}
        <CtaCard />
      </Marquee>

      {/* Top + bottom gradient fade */}
      <div className="from-background pointer-events-none absolute inset-x-0 top-0 h-1/6 bg-gradient-to-b" />
      <div className="from-background pointer-events-none absolute inset-x-0 bottom-0 h-1/6 bg-gradient-to-t" />
    </div>
  );
}

/* ── Card for marquee ──────────────────────────────────────────── */

function MarqueeRecipeCard({ slug }: { slug: string }) {
  const entry = BNTO_REGISTRY.find((e) => e.slug === slug);
  if (!entry) return null;

  return (
    <RecipeCard href={`/${entry.slug}`} className="w-64">
      <RecipeCardHeader>
        <RecipeCardIcon icon={getBntoIcon(entry.slug)} />
        <RecipeCardCategory>{entry.features[0]}</RecipeCardCategory>
      </RecipeCardHeader>
      <RecipeCardContent>
        <RecipeCardTitle>{entry.h1.replace(/ Online Free$/, "")}</RecipeCardTitle>
        <RecipeCardTags tags={entry.features} limit={3} />
      </RecipeCardContent>
    </RecipeCard>
  );
}

/* ── CTA card ─────────────────────────────────────────────────── */

function CtaCard() {
  return (
    <RecipeCard href="/editor" color="primary" className="w-64">
      <RecipeCardHeader>
        <RecipeCardIcon icon={PlusIcon} onSurface />
        <RecipeCardCategory onSurface>Custom</RecipeCardCategory>
      </RecipeCardHeader>
      <RecipeCardContent>
        <RecipeCardTitle>Create your own</RecipeCardTitle>
        <RecipeCardDescription onSurface>Combine tools into custom workflows</RecipeCardDescription>
      </RecipeCardContent>
    </RecipeCard>
  );
}
