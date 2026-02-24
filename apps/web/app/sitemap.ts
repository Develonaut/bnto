import type { MetadataRoute } from "next";
import { BNTO_REGISTRY } from "#lib/bntoRegistry";
import { BASE_URL } from "#lib/constants";

export default function sitemap(): MetadataRoute.Sitemap {
  const bntoPages = BNTO_REGISTRY.map((entry) => ({
    url: `${BASE_URL}/${entry.slug}`,
    lastModified: new Date(),
    changeFrequency: "monthly" as const,
    priority: 0.8,
  }));

  return [
    {
      url: BASE_URL,
      lastModified: new Date(),
      changeFrequency: "weekly" as const,
      priority: 1.0,
    },
    ...bntoPages,
  ];
}
