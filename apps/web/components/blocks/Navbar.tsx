"use client";

import { useEffect, useState } from "react";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useTheme } from "next-themes";

import {
  BookOpenIcon,
  GithubIcon,
  MenuIcon,
  RotateCcwIcon,
  SunIcon,
  TrafficConeIcon,
  XIcon,
} from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { ThemeToggle } from "@/components/ui/AnimatedThemeToggle";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
import { Container } from "@/components/ui/Container";
import { Menu } from "@/components/ui/Menu";
import { RadialSlider } from "@/components/ui/RadialSlider";
import { Text } from "@/components/ui/Text";
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetTitle,
} from "@/components/ui/sheet";
import { THEME_STORE_DEFAULT_ANGLE, useThemeStore } from "@/lib/stores/theme-store";

const GITHUB_URL = "https://github.com/Develonaut/bnto";

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

function navButtonProps(pathname: string, href: string) {
  return {
    depth: "sm" as const,
    pseudo: pathname === href ? ("active" as const) : undefined,
  };
}

export const Navbar = () => {
  const pathname = usePathname();
  const [open, setOpen] = useState(false);

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
                <Button
                  variant="outline"
                  className="hidden lg:inline-flex"
                  href="/motorway"
                  {...navButtonProps(pathname, "/motorway")}
                >
                  <TrafficConeIcon />
                  Motorway
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
                <Button
                  variant="primary"
                  href="/signin"
                  depth="sm"
                  className="hidden lg:inline-flex"
                >
                  Sign in
                </Button>
              </div>
            </div>
          </Card>
        </Container>
      </div>

      {/* Mobile Navigation Sheet */}
      <MobileNavigationMenu open={open} setOpen={setOpen} />
    </section>
  );
};

const THEME_NAMES: Record<string, string> = {
  light: "Los Angeles",
  sunset: "Monaco",
  dark: "Tokyo",
};

function angleToCardinal(deg: number): string {
  if (deg <= 141) return "NW";
  if (deg <= 158) return "NNW";
  if (deg <= 170) return "N";
  if (deg <= 190) return "N";
  if (deg <= 202) return "NNE";
  if (deg <= 219) return "NE";
  return "NE";
}

function NavThemeMenu() {
  const lightAngle = useThemeStore((s) => s.lightAngle);
  const setLightAngle = useThemeStore((s) => s.setLightAngle);
  const { setTheme, resolvedTheme } = useTheme();

  const isDefault =
    lightAngle === THEME_STORE_DEFAULT_ANGLE && resolvedTheme === "light";

  const handleReset = () => {
    setLightAngle(THEME_STORE_DEFAULT_ANGLE);
    setTheme("light");
  };

  return (
    <Menu>
      <Menu.Trigger variant="outline" size="icon" depth="sm">
        <SunIcon />
        <span className="sr-only">Theme settings</span>
      </Menu.Trigger>
      <Menu.Content className="w-auto p-4" offset="lg">
        <div className="flex flex-col items-center gap-3">
          {/* Theme toggle + city name */}
          <div className="flex w-full items-center gap-3">
            <ThemeToggle depth="sm" />
            <Text size="sm" className="w-full font-medium">
              <ThemeName />
            </Text>
          </div>
          {/* Light direction dial */}
          <RadialSlider
            min={135}
            max={225}
            value={lightAngle}
            onChange={setLightAngle}
            startAngle={270}
            endAngle={90}
            size={128}
            strokeWidth={5}
            aria-label="Light direction"
            renderThumb={({ isDragging }) => (
              <Button
                variant="warning"
                size="icon-sm"
                depth="sm"
                pseudo={isDragging ? "active" : undefined}
                className="pointer-events-none size-7"
              >
                <SunIcon className="size-3.5" />
              </Button>
            )}
          >
            <span className="text-xs font-mono font-medium text-muted-foreground">
              {angleToCardinal(lightAngle)}
            </span>
          </RadialSlider>
          {/* Reset — wrapper reserves h-8 space, button bounces in */}
          <div className="h-8 w-full">
            {!isDefault && (
              <Animate.ScaleIn from={0.6} easing="spring-bouncy">
                <Button
                  variant="muted"
                  size="sm"
                  onClick={handleReset}
                  className="w-full"
                >
                  <RotateCcwIcon />
                  Reset
                </Button>
              </Animate.ScaleIn>
            )}
          </div>
        </div>
      </Menu.Content>
    </Menu>
  );
}

function ThemeName() {
  const { resolvedTheme } = useTheme();
  return <>{THEME_NAMES[resolvedTheme ?? "light"] ?? "Los Angeles"}</>;
}

function MobileNavigationMenu({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) {
  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetContent
        aria-describedby={undefined}
        side="top"
        className="inset-0 z-[100] h-dvh w-full bg-primary text-primary-foreground [&>button]:hidden"
      >
        <div className="flex-1 overflow-y-auto">
          <Container className="pb-12">
            {/* Visually hidden title for accessibility */}
            <div className="absolute -m-px h-px w-px overflow-hidden border-0 p-0">
              <SheetTitle className="text-primary">Navigation</SheetTitle>
            </div>

            {/* Close button */}
            <div className="flex justify-end pt-5">
              <SheetClose asChild>
                <Button variant="secondary" size="icon">
                  <XIcon />
                  <span className="sr-only">Close menu</span>
                </Button>
              </SheetClose>
            </div>

            <div className="flex h-full flex-col justify-between gap-20 pt-16">
              {/* Recipes */}
              <div className="flex flex-col gap-10">
                <div className="text-2xl font-display font-bold text-primary-foreground">
                  Recipes
                </div>
                <div className="grid w-full grid-cols-2 gap-x-4 gap-y-10">
                  {RECIPES.map((category) => (
                    <div
                      key={category.title}
                      className="flex flex-col gap-4 text-primary-foreground"
                    >
                      <div className="text-xs uppercase tracking-wider text-white/60">
                        {category.title}
                      </div>
                      <ul className="flex flex-col gap-3">
                        {category.links.map((link) => (
                          <li key={link.url}>
                            <Link
                              href={link.url}
                              onClick={() => setOpen(false)}
                              className="text-lg leading-normal font-medium text-primary-foreground"
                            >
                              {link.label}
                            </Link>
                          </li>
                        ))}
                      </ul>
                    </div>
                  ))}
                </div>
              </div>

              {/* Bottom section */}
              <div className="flex flex-col gap-6">
                <div className="flex items-center gap-4">
                  <Button
                    variant="outline"
                    href="/signin"
                    onClick={() => setOpen(false)}
                  >
                    Sign in
                  </Button>
                  <Button
                    variant="secondary"
                    size="icon"
                    href={GITHUB_URL}
                    target="_blank"
                    rel="noopener noreferrer"
                  >
                    <GithubIcon />
                    <span className="sr-only">GitHub</span>
                  </Button>
                </div>
              </div>
            </div>
          </Container>
        </div>
      </SheetContent>
    </Sheet>
  );
}
