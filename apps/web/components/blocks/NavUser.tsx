"use client";

import { useRouter } from "next/navigation";
import { core } from "@bnto/core";
import { SIGNOUT_COOKIE } from "@bnto/core/constants";

import { CircleUserIcon, LogOutIcon } from "@/components/ui/icons";
import { Button } from "@/components/ui/Button";
import { Menu } from "@/components/ui/Menu";
import { Text } from "@/components/ui/Text";

/**
 * Auth-aware navbar component.
 *
 * - Unauthenticated or anonymous: "Sign In" button linking to /signin
 * - Loading: disabled skeleton button
 * - Authenticated (with email): user icon menu with email + sign out
 *
 * Anonymous users have a Convex session (isAuthenticated: true) but no email.
 * We treat them as unauthenticated for UI purposes — show "Sign In", not
 * the user menu. When they click Sign In, we set the signout signal cookie
 * so the proxy lets them through to /signin (since the proxy sees their
 * anonymous session as "authenticated").
 */
export function NavUser() {
  const { isAuthenticated, isLoading, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  function handleSignOut() {
    signOut();
    router.replace("/signin");
  }

  // Anonymous users have a session but no email — they need the signout
  // signal cookie to bypass the proxy's "auth on /signin" redirect.
  function handleSignInClick() {
    if (isAuthenticated) {
      document.cookie = `${SIGNOUT_COOKIE}=1; path=/; max-age=10; samesite=lax`;
    }
  }

  if (isLoading) {
    return (
      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        disabled
        aria-label="Loading account"
        data-testid="nav-user-loading"
      >
        <CircleUserIcon />
      </Button>
    );
  }

  // Show "Sign In" for unauthenticated users AND anonymous users (no email)
  if (!user?.email) {
    return (
      <Button
        variant="primary"
        elevation="sm"
        href="/signin"
        onClick={handleSignInClick}
        data-testid="nav-sign-in"
      >
        Sign In
      </Button>
    );
  }

  return (
    <Menu>
      <Menu.Trigger
        variant="outline"
        size="icon"
        elevation="sm"
        aria-label="Account menu"
        data-testid="nav-user-menu"
      >
        <CircleUserIcon />
      </Menu.Trigger>
      <Menu.Content className="w-56 p-2" offset="lg">
        <div className="flex flex-col gap-1">
          {/* User info */}
          <div className="px-3 py-2">
            {user.name && (
              <Text size="sm" className="font-medium">
                {user.name}
              </Text>
            )}
            {user.email && (
              <Text size="xs" color="muted">
                {user.email}
              </Text>
            )}
          </div>

          <div className="h-px bg-border" />

          {/* Sign Out */}
          <button
            type="button"
            onClick={handleSignOut}
            data-testid="nav-sign-out"
            className="flex w-full items-center gap-2 rounded-lg px-3 py-2 text-sm transition-colors select-none hover:bg-muted focus:bg-muted"
          >
            <LogOutIcon className="size-4" />
            Sign out
          </button>
        </div>
      </Menu.Content>
    </Menu>
  );
}
