import type { MetadataRoute } from "next";
import { buildBntoSitemapEntries } from "./buildBntoSitemapEntries";
import { buildStaticSitemapEntries } from "./buildStaticSitemapEntries";

export default function sitemap(): MetadataRoute.Sitemap {
  return [...buildStaticSitemapEntries(), ...buildBntoSitemapEntries()];
}
