"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { core } from "@bnto/core";

import {
  Button,
  Container,
  GithubIcon,
  LogOutIcon,
  Row,
  Sheet,
  SheetContent,
  SheetTitle,
  SheetClose,
  Stack,
  Text,
  XIcon,
} from "@bnto/ui";

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
    <Sheet open={open} onOpenChange={onOpenChange}>
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

            <Stack className="h-full justify-between gap-20 pt-16">
              {/* Explore */}
              <Stack className="gap-10">
                <div className="text-2xl font-display font-bold text-primary-foreground">
                  Explore
                </div>
                <div className="grid w-full grid-cols-2 gap-x-4 gap-y-10">
                  {RECIPES.map((category) => (
                    <Stack key={category.title} className="gap-4 text-primary-foreground">
                      <div className="text-xs uppercase tracking-wider text-primary-foreground/60">
                        {category.title}
                      </div>
                      <Stack as="ul" className="gap-3">
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
                      </Stack>
                    </Stack>
                  ))}
                </div>
              </Stack>

              {/* Bottom section */}
              <Stack className="gap-6">
                <Row className="gap-4">
                  <Button variant="outline" href="/editor" onClick={() => onOpenChange(false)}>
                    Create
                  </Button>
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
                </Row>

                {/* Auth section */}
                <div className="h-px bg-primary-foreground/20" />
                {isAuthenticated && user?.email ? (
                  <Stack className="gap-3">
                    {(user.name || user.email) && (
                      <div>
                        {user.name && (
                          <Text size="sm" className="font-medium text-primary-foreground">
                            {user.name}
                          </Text>
                        )}
                        {user.email && (
                          <Text size="xs" className="text-primary-foreground/60">
                            {user.email}
                          </Text>
                        )}
                      </div>
                    )}
                    <Button variant="outline" onClick={handleSignOut} data-testid="mobile-sign-out">
                      <LogOutIcon />
                      Sign out
                    </Button>
                  </Stack>
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
              </Stack>
            </Stack>
          </Container>
        </div>
      </SheetContent>
    </Sheet>
  );
}
