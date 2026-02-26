"use client";

import { useEffect, useState } from "react";

import Link from "next/link";
import { usePathname } from "next/navigation";

import { BookOpenIcon, GithubIcon, MenuIcon } from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Container } from "@/components/ui/Container";
import { Menu } from "@/components/ui/Menu";
import { GITHUB_URL } from "@/lib/copy";
import { isAuthPath } from "@/lib/routes";

import { MobileNavMenu } from "./MobileNavMenu";
import { NavThemeMenu } from "./NavThemeMenu";
import { NavUser } from "./NavUser";

// FIXME: Make contstan.ts file to house magic numbers and values.
// Also this type of stuff should be doabel via css. PLease research examples in the shadcn-blocks
// repo to see how switching from mobile and desktop nav is handled.
// /Users/ryan/Code/shadcn-blocks
const MOBILE_BREAKPOINT = 1024;

interface RecipeLink {
  label: string;
  description: string;
  url: string;
}

interface RecipeCategory {
  title: string;
  links: RecipeLink[];
}

const RECIPES: RecipeCategory[] = [
  {
    title: "Image",
    links: [
      {
        label: "Compress Images",
        description: "Shrink PNG, JPEG, and WebP without losing quality",
        url: "/compress-images",
      },
      {
        label: "Resize Images",
        description: "Scale images to exact dimensions or percentages",
        url: "/resize-images",
      },
      {
        label: "Convert Format",
        description: "Switch between PNG, JPEG, WebP, and GIF",
        url: "/convert-image-format",
      },
    ],
  },
  {
    title: "Data",
    links: [
      {
        label: "Clean CSV",
        description: "Remove empty rows, trim whitespace, deduplicate",
        url: "/clean-csv",
      },
      {
        label: "Rename Columns",
        description: "Rename column headers in bulk",
        url: "/rename-csv-columns",
      },
    ],
  },
  {
    title: "File",
    links: [
      {
        label: "Rename Files",
        description: "Batch rename files with patterns",
        url: "/rename-files",
      },
    ],
  },
];

function navButtonProps(
  pathname: string,
  href: string,
  setPendingHref: (href: string) => void,
  pendingHref: string | null,
) {
  return {
    depth: "sm" as const,
    pressed: pathname === href || pendingHref === href,
    onClick: () => setPendingHref(href),
  };
}

export const Navbar = () => {
  const pathname = usePathname();
  const [open, setOpen] = useState(false);
  // FIXME: This is a code smell.
  const [pendingHref, setPendingHref] = useState<string | null>(null);

  // Clear pending state once the pathname catches up.
  // Derived during render — no effect needed.
  if (pendingHref && pathname === pendingHref) {
    setPendingHref(null);
  }

  const hidden = isAuthPath(pathname);

  useEffect(() => {
    const handleResize = () => {
      if (window.innerWidth > MOBILE_BREAKPOINT) {
        setOpen(false);
      }
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  useEffect(() => {
    document.body.style.overflow = open ? "hidden" : "auto";
  }, [open]);

  // Hide navbar on auth screens (signin, signup)
  // FIXME: Our routes/layout should be compsable enough for this to be done via composition instead.
  if (hidden) return null;

  return (
    <section>
      <div className="fixed top-0 z-50 flex w-full justify-center pt-4">
        <Container size="lg">
          <Card className="rounded-full" depth="sm">
            <div className="flex items-center justify-between gap-3.5 px-6 py-3">
              {/* Logo */}
              <Link
                href="/"
                className="flex max-h-8 items-center gap-2 text-xl font-display font-black tracking-tighter"
              >
                bnto
              </Link>

              {/* Desktop Navigation */}
              <div className="hidden items-center gap-2 lg:flex">
                <Menu>
                  <Menu.Trigger variant="outline" depth="sm">
                    <BookOpenIcon />
                    Recipes
                  </Menu.Trigger>
                  <Menu.Content className="w-[28rem] p-3" offset="lg">
                    <ul className="grid grid-cols-2 gap-1">
                      {RECIPES.map((category) => (
                        <li key={category.title} className="col-span-2">
                          <div className="px-3 pt-3 pb-1 text-xs font-medium uppercase tracking-wider text-muted-foreground">
                            {category.title}
                          </div>
                          <ul className="grid grid-cols-2 gap-1">
                            {category.links.map((link) => (
                              <li key={link.url}>
                                <Link
                                  href={link.url}
                                  className="flex flex-col gap-1 rounded-lg px-3 py-2.5 no-underline outline-hidden transition-colors select-none hover:bg-muted focus:bg-muted"
                                >
                                  <span className="text-sm leading-normal font-medium">
                                    {link.label}
                                  </span>
                                  <span className="text-muted-foreground text-xs leading-normal">
                                    {link.description}
                                  </span>
                                </Link>
                              </li>
                            ))}
                          </ul>
                        </li>
                      ))}
                    </ul>
                  </Menu.Content>
                </Menu>
                {/* FIXME: Make reusable NavButton for buttons in the Nav */}
                <Button
                  variant="outline"
                  href="/pricing"
                  {...navButtonProps(
                    pathname,
                    "/pricing",
                    setPendingHref,
                    pendingHref,
                  )}
                >
                  Pricing
                </Button>
                <Button
                  variant="outline"
                  href="/faq"
                  {...navButtonProps(
                    pathname,
                    "/faq",
                    setPendingHref,
                    pendingHref,
                  )}
                >
                  FAQ
                </Button>
              </div>

              {/* Right side */}
              <div className="flex items-center gap-2">
                <div className="lg:hidden">
                  <Button
                    variant="secondary"
                    size="icon"
                    depth="sm"
                    onClick={() => setOpen(!open)}
                  >
                    <MenuIcon />
                    <span className="sr-only">Open menu</span>
                  </Button>
                </div>
                <Button
                  variant="outline"
                  size="icon"
                  depth="sm"
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
      <MobileNavMenu open={open} onOpenChange={setOpen} recipes={RECIPES} />
    </section>
  );
};
