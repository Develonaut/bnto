"use client";

import { useRouter } from "next/navigation";
import { core } from "@bnto/core";

import {
  CircleUserIcon,
  LogInIcon,
  LogOutIcon,
} from "@/components/ui/icons";
import { Menu } from "@/components/ui/Menu";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

/**
 * Auth-aware navbar component.
 *
 * Always renders the same icon button trigger — never disabled, never changes.
 * Auth state is resolved inside the dropdown menu:
 * - Loading: skeleton placeholder
 * - Unauthenticated or anonymous: "Sign in" menu item
 * - Authenticated (real account): user info + "Sign out"
 *
 * This prevents the button from flashing or changing state while auth loads.
 * The menu gates the check — the trigger is always stable.
 *
 * Anonymous users have a Convex session (isAuthenticated: true) but
 * user.isAnonymous=true. We treat them as unauthenticated for UI purposes.
 * Auth pages are public — no cookie bypass needed for navigation to /signin.
 */
export function NavUser() {
  const { isLoading, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  const isSignedIn = !user?.isAnonymous && !!user?.email;

  function handleSignOut() {
    signOut();
    router.replace("/signin");
  }

  function handleSignIn() {
    router.push("/signin");
  }

  return (
    <Menu>
      <Menu.Trigger
        variant="primary"
        size="icon"
        elevation="sm"
        aria-label="Account menu"
        data-testid="nav-user-menu"
      >
        <CircleUserIcon />
      </Menu.Trigger>
      <Menu.Content className="w-56 p-2" offset="lg">
        <Stack className="gap-1">
          {isLoading ? (
            <div className="px-3 py-2">
              <Skeleton className="h-4 w-24" />
              <Skeleton className="mt-1.5 h-3 w-36" />
            </div>
          ) : isSignedIn ? (
            <>
              <div className="px-3 py-2">
                {user.name && (
                  <Text size="sm" className="font-medium">
                    {user.name}
                  </Text>
                )}
                <Text size="xs" color="muted">
                  {user.email}
                </Text>
              </div>

              <Menu.Separator />

              <Menu.Item onClick={handleSignOut} data-testid="nav-sign-out">
                <LogOutIcon />
                Sign out
              </Menu.Item>
            </>
          ) : (
            <Menu.Item onClick={handleSignIn} data-testid="nav-sign-in">
              <LogInIcon />
              Sign in
            </Menu.Item>
          )}
        </Stack>
      </Menu.Content>
    </Menu>
  );
}
