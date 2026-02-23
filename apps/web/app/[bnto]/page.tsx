import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { Footer } from "@/components/blocks/footer";
import { BNTO_REGISTRY, getBntoBySlug } from "../../lib/bnto-registry";
import { BntoJsonLd } from "./_components/BntoJsonLd";
import { BntoPageShell } from "./_components/BntoPageShell";

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
    title: entry.title,
    description: entry.description,
    openGraph: {
      title: entry.title,
      description: entry.description,
    },
  };
}

/**
 * Bnto tool page -- public, SEO-indexed, anonymous-first.
 *
 * Each slug maps to a predefined workflow via the bnto registry.
 * Unknown slugs return a proper 404.
 *
 * Renders its own Footer with the tool shell replacing the default CTA,
 * so the root layout Footer is hidden via the bnto layout.
 */
export default async function BntoPage({
  params,
}: {
  params: Promise<{ bnto: string }>;
}) {
  const { bnto: slug } = await params;
  const entry = getBntoBySlug(slug);
  if (!entry) notFound();

  return (
    <>
      <BntoJsonLd entry={entry} />
      <main />
      <Footer>
        <BntoPageShell entry={entry} />
      </Footer>
    </>
  );
}
