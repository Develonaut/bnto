"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { core } from "@bnto/core";

import { GithubIcon, LogOutIcon, XIcon } from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Container } from "@/components/ui/Container";
import { Sheet } from "@/components/ui/Sheet";
import { Text } from "@/components/ui/Text";
import { GITHUB_URL } from "@/lib/copy";
import { RECIPES, PAGE_LINKS } from "./navData";

export function MobileNavMenu({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const { isAuthenticated, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  function handleSignOut() {
    signOut();
    onOpenChange(false);
    router.replace("/signin");
  }

  return (
    <Sheet.Root open={open} onOpenChange={onOpenChange}>
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
                              onClick={() => onOpenChange(false)}
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
                  {PAGE_LINKS.map((link) => (
                    <Button
                      key={link.href}
                      variant="outline"
                      href={link.href}
                      onClick={() => onOpenChange(false)}
                    >
                      {link.label}
                    </Button>
                  ))}
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

                {/* Auth section */}
                <div className="h-px bg-white/20" />
                {isAuthenticated && user?.email ? (
                  <div className="flex flex-col gap-3">
                    {(user.name || user.email) && (
                      <div>
                        {user.name && (
                          <Text
                            size="sm"
                            className="font-medium text-primary-foreground"
                          >
                            {user.name}
                          </Text>
                        )}
                        {user.email && (
                          <Text size="xs" className="text-white/60">
                            {user.email}
                          </Text>
                        )}
                      </div>
                    )}
                    <Button
                      variant="outline"
                      onClick={handleSignOut}
                      data-testid="mobile-sign-out"
                    >
                      <LogOutIcon />
                      Sign out
                    </Button>
                  </div>
                ) : (
                  <Button
                    variant="secondary"
                    href="/signin"
                    onClick={() => onOpenChange(false)}
                    data-testid="mobile-sign-in"
                  >
                    Sign In
                  </Button>
                )}
              </div>
            </div>
          </Container>
        </div>
      </Sheet.Content>
    </Sheet.Root>
  );
}
