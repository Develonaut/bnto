import type { LucideIcon } from "@/components/ui/icons";
import {
  ArrowRightLeftIcon,
  Columns3Icon,
  Minimize2Icon,
  PenLineIcon,
  ScalingIcon,
  SparklesIcon,
} from "@/components/ui/icons";

const BNTO_ICONS: Record<string, LucideIcon> = {
  "compress-images": Minimize2Icon,
  "resize-images": ScalingIcon,
  "convert-image-format": ArrowRightLeftIcon,
  "rename-files": PenLineIcon,
  "clean-csv": SparklesIcon,
  "rename-csv-columns": Columns3Icon,
};

export function getBntoIcon(slug: string): LucideIcon {
  return BNTO_ICONS[slug] ?? SparklesIcon;
}
