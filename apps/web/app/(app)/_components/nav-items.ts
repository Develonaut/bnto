import {
  LayoutDashboardIcon,
  WorkflowIcon,
  PlayIcon,
  SettingsIcon,
} from "@bnto/ui";
import type { LucideIcon } from "@bnto/ui";

export interface NavItem {
  label: string;
  href: string;
  icon: LucideIcon;
}

/** Flat nav items for the top bar. Shared by desktop and mobile nav. */
export const NAV_ITEMS: NavItem[] = [
  { label: "Dashboard", href: "/", icon: LayoutDashboardIcon },
  { label: "Workflows", href: "/workflows", icon: WorkflowIcon },
  { label: "Executions", href: "/executions", icon: PlayIcon },
  { label: "Settings", href: "/settings", icon: SettingsIcon },
];
