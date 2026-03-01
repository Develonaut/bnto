"use client";

import { useRouter } from "next/navigation";
import { core } from "@bnto/core";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";

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
 * - Authenticated (with email): user info + "Sign out"
 *
 * This prevents the button from flashing or changing state while auth loads.
 * The menu gates the check — the trigger is always stable.
 *
 * Anonymous users have a Convex session (isAuthenticated: true) but no email.
 * We treat them as unauthenticated for UI purposes. When they click Sign In,
 * we set the signout signal cookie so the proxy lets them through to /signin
 * (since the proxy sees their anonymous session as "authenticated").
 */
export function NavUser() {
  const { isAuthenticated, isLoading, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  const isSignedIn = !!user?.email;

  function handleSignOut() {
    signOut();
    router.replace("/signin");
  }

  function handleSignIn() {
    // Anonymous users have a session but no email — they need the signout
    // signal cookie to bypass the proxy's "auth on /signin" redirect.
    if (isAuthenticated) {
      document.cookie = `${SIGNOUT_COOKIE}=1; path=/; max-age=10; samesite=lax`;
    }
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
