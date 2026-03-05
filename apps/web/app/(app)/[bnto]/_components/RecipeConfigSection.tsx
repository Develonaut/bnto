"use client";

import { Animate } from "@bnto/ui";
import { CleanCsvConfig } from "./configs/CleanCsvConfig";
import { CompressImagesConfig } from "./configs/CompressImagesConfig";
import { ConvertFormatConfig } from "./configs/ConvertFormatConfig";
import { RenameCsvColumnsConfig } from "./configs/RenameCsvColumnsConfig";
import { RenameFilesConfig } from "./configs/RenameFilesConfig";
import { ResizeImagesConfig } from "./configs/ResizeImagesConfig";
import type { BntoConfigMap, BntoSlug } from "./configs/types";

interface RecipeConfigSectionProps {
  slug: string;
  config: BntoConfigMap[BntoSlug];
  onChange: (config: BntoConfigMap[BntoSlug]) => void;
}

/**
 * Recipe-specific configuration rendered inline.
 * Routes slug to the correct config component. Returns null for unknown slugs.
 */
export function RecipeConfigSection({ slug, config, onChange }: RecipeConfigSectionProps) {
  const content = renderConfig(slug, config, onChange);
  if (!content) return null;

  return (
    <Animate.FadeIn>
      <div>{content}</div>
    </Animate.FadeIn>
  );
}

function renderConfig(
  slug: string,
  config: BntoConfigMap[BntoSlug],
  onChange: (config: BntoConfigMap[BntoSlug]) => void,
) {
  switch (slug) {
    case "compress-images":
      return <CompressImagesConfig value={config as BntoConfigMap["compress-images"]} onChange={onChange} />;
    case "resize-images":
      return <ResizeImagesConfig value={config as BntoConfigMap["resize-images"]} onChange={onChange} />;
    case "convert-image-format":
      return <ConvertFormatConfig value={config as BntoConfigMap["convert-image-format"]} onChange={onChange} />;
    case "rename-files":
      return <RenameFilesConfig value={config as BntoConfigMap["rename-files"]} onChange={onChange} />;
    case "clean-csv":
      return <CleanCsvConfig value={config as BntoConfigMap["clean-csv"]} onChange={onChange} />;
    case "rename-csv-columns":
      return <RenameCsvColumnsConfig />;
    default:
      return null;
  }
}
