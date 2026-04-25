import { Container, Toolbar, ToolbarGroup } from "@bnto/ui";

import { DesktopNav } from "./DesktopNav";
import { NavButton } from "./NavButton";
import { NavbarActions } from "./NavbarActions";

export const Navbar = () => {
  return (
    <section data-testid="navbar">
      <div className="fixed top-0 z-sticky flex w-full justify-center pt-4">
        <Container size="lg">
          <Toolbar>
            {/* Logo — flex-1 so left/right columns balance for true centering */}
            <ToolbarGroup className="min-w-0 flex-1">
              <NavButton
                href="/"
                elevation="sm"
                data-testid="nav-link-home"
                className="text-xl font-display font-black tracking-tighter"
              >
                {/* eslint-disable-next-line @next/next/no-img-element -- inline SVG logo, next/image not needed */}
                <img
                  src="/favicon/Logo.svg"
                  alt=""
                  width={24}
                  height={24}
                  className="shrink-0"
                  aria-hidden
                />
                bnto
              </NavButton>
            </ToolbarGroup>

            <DesktopNav />
            <NavbarActions />
          </Toolbar>
        </Container>
      </div>
    </section>
  );
};
