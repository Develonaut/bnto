/** Icon string → Lucide component resolver for visual consumers. */

import type { LucideIcon } from "@bnto/ui";
import {
  ImageIcon,
  SheetIcon,
  FolderOpenIcon,
  ArrowLeftRightIcon,
  PenLineIcon,
  GlobeIcon,
  TerminalIcon,
  RepeatIcon,
  GitForkIcon,
  BoxIcon,
  // I/O contextual icons (from getNodeIcon)
  FileUpIcon,
  TextCursorInputIcon,
  LinkIcon,
  DownloadIcon,
  MonitorIcon,
  EyeIcon,
} from "@bnto/ui";

/**
 * Maps icon identifier strings to Lucide components.
 *
 * getNodeIcon() in @bnto/nodes returns these strings.
 * This registry resolves them to renderable components.
 */
const ICON_COMPONENTS: Record<string, LucideIcon> = {
  // Processing node icons (static from NODE_TYPE_INFO)
  image: ImageIcon,
  sheet: SheetIcon,
  "folder-open": FolderOpenIcon,
  "arrow-left-right": ArrowLeftRightIcon,
  "pen-line": PenLineIcon,
  globe: GlobeIcon,
  terminal: TerminalIcon,
  repeat: RepeatIcon,
  "git-fork": GitForkIcon,
  box: BoxIcon,
  // I/O contextual icons (from getNodeIcon)
  "file-up": FileUpIcon,
  "text-cursor-input": TextCursorInputIcon,
  link: LinkIcon,
  download: DownloadIcon,
  monitor: MonitorIcon,
  eye: EyeIcon,
};

export { ICON_COMPONENTS };
