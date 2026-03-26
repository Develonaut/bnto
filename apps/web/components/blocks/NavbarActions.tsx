import { Button, GithubIcon, ToolbarGroup } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";
import { MobileNav } from "./MobileNav";
import { NavThemeMenu } from "./NavThemeMenu";
import { NavUser } from "./NavUser";

/** Right-side navbar actions — mobile nav, GitHub, theme toggle, user menu. */
export function NavbarActions() {
  return (
    <ToolbarGroup className="min-w-0 flex-1 justify-end gap-2">
      <MobileNav />
      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        href={GITHUB_URL}
        target="_blank"
        rel="noopener noreferrer"
        className="hidden lg:inline-flex"
      >
        <GithubIcon />
        <span className="sr-only">GitHub</span>
      </Button>
      <NavThemeMenu />
      <NavUser />
    </ToolbarGroup>
  );
}
