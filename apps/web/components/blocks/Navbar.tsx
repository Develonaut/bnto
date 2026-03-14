import { Container, Toolbar, ToolbarGroup } from "@bnto/ui";

import { DesktopNav } from "./DesktopNav";
import { MobileNav } from "./MobileNav";
import { NavButton } from "./NavButton";
import { NavThemeMenu } from "./NavThemeMenu";
import { NavUser } from "./NavUser";

export const Navbar = () => {
  return (
    <section>
      <div className="fixed top-0 z-sticky flex w-full justify-center pt-4">
        <Container size="lg">
          <Toolbar>
            {/* Logo — flex-1 so left/right columns balance for true centering */}
            <ToolbarGroup className="min-w-0 flex-1">
              <NavButton
                href="/"
                elevation="sm"
                className="text-xl font-display font-black tracking-tighter"
              >
                bnto
              </NavButton>
            </ToolbarGroup>

            <DesktopNav />

            {/* Right side — flex-1 mirrors the logo column */}
            <ToolbarGroup className="min-w-0 flex-1 justify-end gap-2">
              <MobileNav />
              <NavThemeMenu />
              <NavUser />
            </ToolbarGroup>
          </Toolbar>
        </Container>
      </div>
    </section>
  );
};
