/**
 * Category → icon mapping shared across all recipe card surfaces.
 */

import type { LucideIcon } from "@bnto/ui";
import { FileIcon, GlobeIcon, ImageIcon, SheetIcon } from "@bnto/ui";

export const CATEGORY_ICON: Record<string, LucideIcon> = {
  image: ImageIcon,
  file: FileIcon,
  spreadsheet: SheetIcon,
  network: GlobeIcon,
};
