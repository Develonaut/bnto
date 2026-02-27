"use client";

import { useEffect, useState } from "react";

import Link from "next/link";
import { usePathname } from "next/navigation";

import { GithubIcon, MenuIcon } from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Container } from "@/components/ui/Container";
import { GITHUB_URL } from "@/lib/copy";

import { DesktopNav } from "./DesktopNav";
import { MobileNavMenu } from "./MobileNavMenu";
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
  const pathname = usePathname();
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
          <Card className="rounded-full" elevation="sm">
            <div className="flex items-center justify-between gap-3.5 px-6 py-3">
              {/* Logo */}
              <Link
                href="/"
                className="flex max-h-8 items-center gap-2 text-xl font-display font-black tracking-tighter"
              >
                bnto
              </Link>

              <DesktopNav pathname={pathname} />

              {/* Right side */}
              <div className="flex items-center gap-2">
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
              </div>
            </div>
          </Card>
        </Container>
      </div>

      {/* Mobile Navigation Sheet */}
      <MobileNavMenu open={mobileOpen} onOpenChange={setMobileOpen} />
    </section>
  );
};
