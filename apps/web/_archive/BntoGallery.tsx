import Link from "next/link";
import { ArrowRightIcon } from "@bnto/ui";
import { SectionLabel } from "@bnto/ui/section-label";
import { Card } from "@bnto/ui/card";
import { BNTO_REGISTRY } from "../../../lib/bnto-registry";
import type { BntoEntry } from "../../../lib/bnto-registry";

/**
 * Tools showcase grid with section heading.
 *
 * Server component — reads from a static registry, no hooks needed.
 */
export function BntoGallery() {
  return (
    <section id="tools" className="container flex flex-col gap-12 py-12 lg:py-16">
      <SectionLabel>Tools</SectionLabel>

      <div className="grid gap-4 lg:grid-cols-2 lg:gap-8">
        <h2 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
          Tools that just work
        </h2>
        <p className="text-muted-foreground text-lg leading-relaxed text-balance">
          Drop your files, get results. Every tool runs in your browser — no
          server uploads, no waiting, no account required.
        </p>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {BNTO_REGISTRY.map((entry) => (
          <BntoCard key={entry.slug} entry={entry} />
        ))}
      </div>
    </section>
  );
}

function BntoCard({ entry }: { entry: BntoEntry }) {
  return (
    <Link href={`/${entry.slug}`} className="group">
      <Card className="flex h-full flex-col p-6 motion-safe:transition-all motion-safe:hover:-translate-y-0.5 hover:shadow-lg">
        <div className="flex items-start justify-between">
          <h3 className="font-display text-lg font-semibold text-foreground group-hover:text-primary">
            {entry.h1}
          </h3>
          <ArrowRightIcon className="text-muted-foreground size-4 shrink-0 opacity-0 transition-all group-hover:translate-x-0.5 group-hover:opacity-100 group-focus-within:opacity-100" />
        </div>
        <p className="mt-2 flex-1 text-sm text-muted-foreground">
          {entry.description}
        </p>
        <div className="mt-4 flex flex-wrap gap-1.5">
          {entry.features.map((feature) => (
            <span
              key={feature}
              className="rounded-full bg-muted px-2.5 py-0.5 text-xs text-muted-foreground"
            >
              {feature}
            </span>
          ))}
        </div>
      </Card>
    </Link>
  );
}
