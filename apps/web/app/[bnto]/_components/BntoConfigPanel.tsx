"use client";

import { Heading } from "@/components/ui/Heading";
import { CleanCsvConfig } from "./configs/CleanCsvConfig";
import { CompressImagesConfig } from "./configs/CompressImagesConfig";
import { ConvertFormatConfig } from "./configs/ConvertFormatConfig";
import { RenameCsvColumnsConfig } from "./configs/RenameCsvColumnsConfig";
import { RenameFilesConfig } from "./configs/RenameFilesConfig";
import { ResizeImagesConfig } from "./configs/ResizeImagesConfig";
import type { BntoConfigMap, BntoSlug } from "./configs/types";

interface BntoConfigPanelProps {
  slug: string;
  config: BntoConfigMap[BntoSlug];
  onChange: (config: BntoConfigMap[BntoSlug]) => void;
}

/**
 * Renders the context-specific configuration controls for a bnto tool.
 *
 * Routes the slug to the correct config component. Each config
 * component manages its own UI but reports changes up via onChange.
 */
export function BntoConfigPanel({
  slug,
  config,
  onChange,
}: BntoConfigPanelProps) {
  return (
    <div className="rounded-xl border border-border bg-card p-5">
      <Heading level={2} size="xs" className="text-foreground mb-4 text-sm font-semibold">Settings</Heading>
      {renderConfig(slug, config, onChange)}
    </div>
  );
}

function renderConfig(
  slug: string,
  config: BntoConfigMap[BntoSlug],
  onChange: (config: BntoConfigMap[BntoSlug]) => void,
) {
  switch (slug) {
    case "compress-images":
      return (
        <CompressImagesConfig
          value={config as BntoConfigMap["compress-images"]}
          onChange={onChange}
        />
      );
    case "resize-images":
      return (
        <ResizeImagesConfig
          value={config as BntoConfigMap["resize-images"]}
          onChange={onChange}
        />
      );
    case "convert-image-format":
      return (
        <ConvertFormatConfig
          value={config as BntoConfigMap["convert-image-format"]}
          onChange={onChange}
        />
      );
    case "rename-files":
      return (
        <RenameFilesConfig
          value={config as BntoConfigMap["rename-files"]}
          onChange={onChange}
        />
      );
    case "clean-csv":
      return (
        <CleanCsvConfig
          value={config as BntoConfigMap["clean-csv"]}
          onChange={onChange}
        />
      );
    case "rename-csv-columns":
      return <RenameCsvColumnsConfig />;
    default:
      return (
        <p className="text-muted-foreground text-sm">
          No configuration available for this tool.
        </p>
      );
  }
}
