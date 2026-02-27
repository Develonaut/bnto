"use client";

import { Accordion } from "@/components/ui/Accordion";
import { Animate } from "@/components/ui/Animate";
import { Card } from "@/components/ui/Card";
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
 * Recipe-specific configuration wrapped in Motorway styling.
 * Routes slug to the correct config component. Wraps in a Card with
 * a collapsible Accordion (default open). Returns null for unknown slugs.
 */
export function RecipeConfigSection({ slug, config, onChange }: RecipeConfigSectionProps) {
  const content = renderConfig(slug, config, onChange);
  if (!content) return null;

  return (
    <Animate.FadeIn>
      <Card elevation="sm">
        <Accordion type="single" collapsible defaultValue="config">
          <Accordion.Item value="config" className="border-none">
            <Accordion.Trigger className="px-5">Settings</Accordion.Trigger>
            <Accordion.Content className="px-5">
              {content}
            </Accordion.Content>
          </Accordion.Item>
        </Accordion>
      </Card>
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
