import type { LucideIcon } from "@bnto/ui";
import {
  ArrowRightLeftIcon,
  Columns3Icon,
  CropIcon,
  LayersIcon,
  ListChecksIcon,
  Minimize2Icon,
  PenLineIcon,
  ScalingIcon,
  SparklesIcon,
  ZapIcon,
} from "@bnto/ui";

const BNTO_ICONS: Record<string, LucideIcon> = {
  "compress-images": Minimize2Icon,
  "resize-images": ScalingIcon,
  "convert-image-format": ArrowRightLeftIcon,
  "rename-files": PenLineIcon,
  "clean-csv": SparklesIcon,
  "rename-csv-columns": Columns3Icon,
  "optimize-images-for-web": ZapIcon,
  "generate-thumbnails": CropIcon,
  "compress-and-rename": LayersIcon,
  "standardize-csv": ListChecksIcon,
};

export function getBntoIcon(slug: string): LucideIcon {
  return BNTO_ICONS[slug] ?? SparklesIcon;
}
