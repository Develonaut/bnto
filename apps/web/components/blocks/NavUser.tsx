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
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

/**
 * Auth-aware navbar component.
 *
 * Always renders the same icon button trigger to prevent layout shift.
 * The dropdown adapts based on auth state:
 * - Loading: trigger is disabled
 * - Unauthenticated or anonymous: menu has a "Sign in" link
 * - Authenticated (with email): menu shows user info + sign out
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
        disabled={isLoading}
        aria-label={isLoading ? "Loading account" : "Account menu"}
        data-testid={isLoading ? "nav-user-loading" : "nav-user-menu"}
      >
        <CircleUserIcon />
      </Menu.Trigger>
      <Menu.Content className="w-56 p-2" offset="lg">
        <Stack className="gap-1">
          {isSignedIn ? (
            <>
              {/* User info */}
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
