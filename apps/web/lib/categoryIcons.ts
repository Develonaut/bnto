/**
 * Category → icon mapping shared between explore page and my-recipes grid.
 */

import type { LucideIcon } from "@bnto/ui";
import { FileIcon, GlobeIcon, ImageIcon, SheetIcon } from "@bnto/ui";

export const CATEGORY_ICON: Record<string, LucideIcon> = {
  image: ImageIcon,
  file: FileIcon,
  spreadsheet: SheetIcon,
  network: GlobeIcon,
};
