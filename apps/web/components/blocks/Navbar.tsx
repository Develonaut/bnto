"use client";

import { useEffect, useState } from "react";

import { GithubIcon, MenuIcon } from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Container } from "@/components/ui/Container";
import { Toolbar } from "@/components/ui/Toolbar";
import { GITHUB_URL } from "@/lib/copy";

import { DesktopNav } from "./DesktopNav";
import { MobileNavMenu } from "./MobileNavMenu";
import { NavButton } from "./NavButton";
import { NavThemeMenu } from "./NavThemeMenu";
import { NavUser } from "./NavUser";

/**
 * Matches Tailwind's `lg` breakpoint (1024px).
 * Used by the resize handler to close the mobile sheet when the viewport
 * crosses into desktop territory. CSS `hidden lg:flex` handles the visual
 * toggle — this just ensures the Sheet closes so it isn't stuck open behind
 * the desktop nav if the user resizes mid-session.
 */
const LG_BREAKPOINT = 1024;

export const Navbar = () => {
  const [mobileOpen, setMobileOpen] = useState(false);

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth > LG_BREAKPOINT) {
        setMobileOpen(false);
      }
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  useEffect(() => {
    document.body.style.overflow = mobileOpen ? "hidden" : "auto";
  }, [mobileOpen]);

  return (
    <section>
      <div className="fixed top-0 z-50 flex w-full justify-center pt-4">
        <Container size="lg">
          <Toolbar>
            {/* Logo — flex-1 so left/right columns balance for true centering */}
            <Toolbar.Group className="min-w-0 flex-1">
              <NavButton
                href="/"
                elevation="sm"
                className="text-xl font-display font-black tracking-tighter"
              >
                bnto
              </NavButton>
            </Toolbar.Group>

            <DesktopNav />

            {/* Right side — flex-1 mirrors the logo column */}
            <Toolbar.Group className="min-w-0 flex-1 justify-end gap-2">
              <div className="lg:hidden">
                <Button
                  variant="secondary"
                  size="icon"
                  elevation="sm"
                  onClick={() => setMobileOpen(!mobileOpen)}
                >
                  <MenuIcon />
                  <span className="sr-only">Open menu</span>
                </Button>
              </div>
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
            </Toolbar.Group>
          </Toolbar>
        </Container>
      </div>

      {/* Mobile Navigation Sheet */}
      <MobileNavMenu open={mobileOpen} onOpenChange={setMobileOpen} />
    </section>
  );
};
