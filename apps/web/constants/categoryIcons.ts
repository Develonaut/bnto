/**
 * Category → icon mapping shared across all recipe card surfaces.
 */

import type { LucideIcon } from "@bnto/ui";
import { BracesIcon, FileIcon, FileVideoIcon, GlobeIcon, ImageIcon, SheetIcon } from "@bnto/ui";

export const CATEGORY_ICON: Record<string, LucideIcon> = {
  image: ImageIcon,
  file: FileIcon,
  spreadsheet: SheetIcon,
  data: BracesIcon,
  network: GlobeIcon,
  video: FileVideoIcon,
};
