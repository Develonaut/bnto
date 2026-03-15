"use client";

import { Suspense } from "react";
import { FadeIn, Skeleton } from "@bnto/ui";
import type { BntoConfigMap, BntoSlug } from "./configs/types";
import { CONFIG_REGISTRY } from "./configs/registry";

interface RecipeConfigSectionProps {
  slug: string;
  config: BntoConfigMap[BntoSlug];
  onChange: (config: BntoConfigMap[BntoSlug]) => void;
}

/**
 * Recipe-specific configuration rendered inline.
 *
 * Looks up the lazy-loaded config component from the registry.
 * Only the config needed for the current recipe is downloaded.
 */
export function RecipeConfigSection({ slug, config, onChange }: RecipeConfigSectionProps) {
  const ConfigComponent = CONFIG_REGISTRY[slug as BntoSlug];
  if (!ConfigComponent) return null;

  return (
    <FadeIn>
      <Suspense fallback={<Skeleton className="h-10 w-full" />}>
        <div>
          <ConfigComponent value={config} onChange={onChange} />
        </div>
      </Suspense>
    </FadeIn>
  );
}
