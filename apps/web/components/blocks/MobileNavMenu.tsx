"use client";

import Link from "next/link";

import { GithubIcon, XIcon } from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Container } from "@/components/ui/Container";
import { Sheet } from "@/components/ui/Sheet";
import { GITHUB_URL } from "@/lib/copy";

/* ── Types ────────────────────────────────────────────────────── */

interface RecipeLink {
  label: string;
  description: string;
  url: string;
}

interface RecipeCategory {
  title: string;
  links: RecipeLink[];
}

/* ── MobileNavMenu ────────────────────────────────────────────── */

export function MobileNavMenu({
  open,
  setOpen,
  recipes,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
  recipes: RecipeCategory[];
}) {
  return (
    <Sheet.Root open={open} onOpenChange={setOpen}>
      <Sheet.Content
        aria-describedby={undefined}
        side="top"
        className="inset-0 z-[100] h-dvh w-full bg-primary text-primary-foreground [&>button]:hidden"
      >
        <div className="flex-1 overflow-y-auto">
          <Container className="pb-12">
            {/* Visually hidden title for accessibility */}
            <div className="absolute -m-px h-px w-px overflow-hidden border-0 p-0">
              <Sheet.Title className="text-primary">Navigation</Sheet.Title>
            </div>

            {/* Close button */}
            <div className="flex justify-end pt-5">
              <Sheet.Close asChild>
                <Button variant="secondary" size="icon">
                  <XIcon />
                  <span className="sr-only">Close menu</span>
                </Button>
              </Sheet.Close>
            </div>

            <div className="flex h-full flex-col justify-between gap-20 pt-16">
              {/* Recipes */}
              <div className="flex flex-col gap-10">
                <div className="text-2xl font-display font-bold text-primary-foreground">
                  Recipes
                </div>
                <div className="grid w-full grid-cols-2 gap-x-4 gap-y-10">
                  {recipes.map((category) => (
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
                    href="/pricing"
                    onClick={() => setOpen(false)}
                  >
                    Pricing
                  </Button>
                  <Button
                    variant="outline"
                    href="/faq"
                    onClick={() => setOpen(false)}
                  >
                    FAQ
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
      </Sheet.Content>
    </Sheet.Root>
  );
}
