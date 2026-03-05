"use client";

import { useRouter } from "next/navigation";
import { core } from "@bnto/core";

import { CircleUserIcon, LogInIcon, LogOutIcon } from "@bnto/ui";
import {
  Menu,
  MenuTrigger,
  MenuContent,
  MenuSeparator,
  MenuItem,
  Skeleton,
  Stack,
  Text,
} from "@bnto/ui";

/**
 * Auth-aware navbar component.
 *
 * Always renders the same icon button trigger — never disabled, never changes.
 * Auth state is resolved inside the dropdown menu:
 * - Loading: skeleton placeholder
 * - Unauthenticated: "Sign in" menu item
 * - Authenticated: user info + "Sign out"
 *
 * This prevents the button from flashing or changing state while auth loads.
 * The menu gates the check — the trigger is always stable.
 */
export function NavUser() {
  const { isAuthenticated, isLoading, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  function handleSignOut() {
    signOut();
    router.replace("/signin");
  }

  function handleSignIn() {
    router.push("/signin");
  }

  return (
    <Menu>
      <MenuTrigger
        variant="primary"
        size="icon"
        elevation="sm"
        aria-label="Account menu"
        data-testid="nav-user-menu"
      >
        <CircleUserIcon />
      </MenuTrigger>
      <MenuContent className="w-56 p-2" offset="lg">
        <Stack className="gap-1">
          {isLoading ? (
            <div className="px-3 py-2">
              <Skeleton className="h-4 w-24" />
              <Skeleton className="mt-1.5 h-3 w-36" />
            </div>
          ) : isAuthenticated ? (
            <>
              <div className="px-3 py-2">
                {user?.name && (
                  <Text size="sm" className="font-medium">
                    {user.name}
                  </Text>
                )}
                <Text size="xs" color="muted">
                  {user?.email}
                </Text>
              </div>

              <MenuSeparator />

              <MenuItem onClick={handleSignOut} data-testid="nav-sign-out">
                <LogOutIcon />
                Sign out
              </MenuItem>
            </>
          ) : (
            <MenuItem onClick={handleSignIn} data-testid="nav-sign-in">
              <LogInIcon />
              Sign in
            </MenuItem>
          )}
        </Stack>
      </MenuContent>
    </Menu>
  );
}
