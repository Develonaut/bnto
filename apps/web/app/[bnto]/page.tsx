import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { AppShell } from "@/components/ui/AppShell";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { BntoPageShell } from "./_components/BntoPageShell";

/** Only slugs from generateStaticParams are valid — everything else is 404
 * at the routing level (no component code runs for unknown slugs). */
export const dynamicParams = false;

/** Pre-render all registered slugs at build time. */
export function generateStaticParams() {
  return BNTO_REGISTRY.map((bnto) => ({ bnto: bnto.slug }));
}

/** Per-slug metadata -- resolved at build time for static pages. */
export async function generateMetadata({
  params,
}: {
  params: Promise<{ bnto: string }>;
}): Promise<Metadata> {
  const { bnto } = await params;
  const entry = getBntoBySlug(bnto);
  if (!entry) return {};
  return {
    title: { absolute: entry.title },
    description: entry.description,
    openGraph: {
      title: entry.title,
      description: entry.description,
    },
  };
}

export default async function BntoPage({
  params,
}: {
  params: Promise<{ bnto: string }>;
}) {
  const { bnto: slug } = await params;
  const entry = getBntoBySlug(slug);
  if (!entry) notFound();

  return (
    <AppShell.Content>
      <BntoJsonLd entry={entry} />
      <BntoPageShell entry={entry} />
    </AppShell.Content>
  );
}
