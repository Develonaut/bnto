import type { MetadataRoute } from "next";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";
import { BASE_URL } from "@/lib/constants";

/** Generate sitemap entries for all predefined bnto tool pages. */
export function buildBntoSitemapEntries(): MetadataRoute.Sitemap {
  return BNTO_REGISTRY.map((entry) => ({
    url: `${BASE_URL}/${entry.slug}`,
    lastModified: new Date(),
    changeFrequency: "monthly" as const,
    priority: 0.8,
  }));
}
