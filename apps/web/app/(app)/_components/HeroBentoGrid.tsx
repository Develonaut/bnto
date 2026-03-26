import type { ReactNode } from "react";

import { Grid, GridItem, ScaleIn, Stagger } from "@bnto/ui";

import { getBntoBySlug } from "@/lib/bntoRegistry";

import { HeroCtaCard } from "./HeroCtaCard";
import { HeroHeadingCard } from "./HeroHeadingCard";
import { HeroOpenSourceCard } from "./HeroOpenSourceCard";
import { HeroPrivacyCard } from "./HeroPrivacyCard";
import { HeroRecipeCard } from "./HeroRecipeCard";
import { HeroStatCard } from "./HeroStatCard";

/* ── Recipe entries ────────────────────────────────────────────── */

const r = (slug: string) => {
  const entry = getBntoBySlug(slug)!;
  return <HeroRecipeCard entry={entry} />;
};

/* ── Cell descriptors ─────────────────────────────────────────── */

interface Cell {
  content: ReactNode;
  colSpan: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
  rowSpan?: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
  className?: string;
}

const CELLS: Cell[] = [
  /* Row 1-2 */
  {
    content: <HeroHeadingCard />,
    colSpan: 5,
    rowSpan: 2,
    className: "max-sm:col-span-full sm:max-lg:col-span-6 sm:max-lg:row-span-2",
  },
  {
    content: r("compress-images"),
    colSpan: 4,
    className: "max-sm:col-span-full sm:max-lg:col-span-3",
  },
  {
    content: <HeroPrivacyCard />,
    colSpan: 3,
    className: "max-sm:col-span-full sm:max-lg:col-span-3",
  },
  {
    content: r("resize-images"),
    colSpan: 4,
    className: "max-sm:col-span-full sm:max-lg:col-span-3",
  },
  { content: <HeroStatCard />, colSpan: 3, className: "max-sm:col-span-full sm:max-lg:col-span-3" },
  /* Row 3 */
  {
    content: r("convert-image-format"),
    colSpan: 3,
    className: "max-sm:col-span-full sm:max-lg:col-span-3",
  },
  {
    content: r("rename-files"),
    colSpan: 5,
    className: "max-sm:col-span-full sm:max-lg:col-span-3",
  },
  { content: <HeroCtaCard />, colSpan: 4, className: "max-sm:col-span-full sm:max-lg:col-span-6" },
  /* Row 4 */
  {
    content: <HeroOpenSourceCard />,
    colSpan: 4,
    className: "max-sm:col-span-full sm:max-lg:col-span-6",
  },
  { content: r("clean-csv"), colSpan: 4, className: "max-sm:col-span-full sm:max-lg:col-span-3" },
  {
    content: r("rename-csv-columns"),
    colSpan: 4,
    className: "max-sm:col-span-full sm:max-lg:col-span-3",
  },
];

/* ── Grid composition ─────────────────────────────────────────── */

export function HeroBentoGrid() {
  return (
    <Stagger asChild>
      <Grid cols={{ mobile: 1, tablet: 6, desktop: 12 }} gap="md">
        {CELLS.map((cell, i) => (
          <GridItem
            key={i}
            colSpan={cell.colSpan}
            rowSpan={cell.rowSpan}
            className={cell.className}
          >
            <ScaleIn index={i} from={0.9} easing="spring-bouncy" className="h-full">
              {cell.content}
            </ScaleIn>
          </GridItem>
        ))}
      </Grid>
    </Stagger>
  );
}
