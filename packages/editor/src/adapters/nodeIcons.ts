/** Icon string → Lucide component resolver for visual consumers. */

import type { LucideIcon } from "@bnto/ui";
import {
  ImageIcon,
  TableIcon,
  FolderOpenIcon,
  ShuffleIcon,
  PenLineIcon,
  GlobeIcon,
  TerminalSquareIcon,
  BracesIcon,
  RefreshCwIcon,
  Columns3Icon,
  UploadIcon,
  DownloadIcon,
} from "@bnto/ui";

/**
 * Maps icon identifier strings (from NodeTypeInfo.icon) to Lucide components.
 *
 * Consumers look up NODE_TYPE_INFO[type].icon to get the string,
 * then resolve it here to an actual component for rendering.
 */
const ICON_COMPONENTS: Record<string, LucideIcon> = {
  image: ImageIcon,
  table: TableIcon,
  "folder-open": FolderOpenIcon,
  shuffle: ShuffleIcon,
  "pen-line": PenLineIcon,
  globe: GlobeIcon,
  "terminal-square": TerminalSquareIcon,
  braces: BracesIcon,
  "refresh-cw": RefreshCwIcon,
  "columns-3": Columns3Icon,
  upload: UploadIcon,
  download: DownloadIcon,
};

export { ICON_COMPONENTS };
