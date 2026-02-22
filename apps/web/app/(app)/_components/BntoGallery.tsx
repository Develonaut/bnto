import Link from "next/link";
import { BNTO_REGISTRY } from "../../../lib/bnto-registry";
import type { BntoEntry } from "../../../lib/bnto-registry";

/**
 * Grid of bnto tool cards from the registry.
 *
 * Each card links to the tool's public URL. Server component --
 * reads from a static registry, no hooks needed.
 */
export function BntoGallery() {
  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {BNTO_REGISTRY.map((entry) => (
        <BntoCard key={entry.slug} entry={entry} />
      ))}
    </div>
  );
}

function BntoCard({ entry }: { entry: BntoEntry }) {
  return (
    <Link
      href={`/${entry.slug}`}
      className="group rounded-lg border border-border bg-card p-6 motion-safe:transition-shadow hover:shadow-md"
    >
      <h2 className="text-lg font-semibold text-foreground group-hover:text-primary">
        {entry.h1}
      </h2>
      <p className="mt-2 text-sm text-muted-foreground">{entry.description}</p>
      <div className="mt-4 flex flex-wrap gap-1.5">
        {entry.features.map((feature) => (
          <span
            key={feature}
            className="rounded-sm bg-muted px-2 py-0.5 text-xs text-muted-foreground"
          >
            {feature}
          </span>
        ))}
      </div>
    </Link>
  );
}
