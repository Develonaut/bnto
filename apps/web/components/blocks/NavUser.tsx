"use client";

import { useRouter } from "next/navigation";
import { core } from "@bnto/core";

import { CircleUserIcon } from "@bnto/ui";
import { Menu, MenuTrigger, MenuContent, Stack } from "@bnto/ui";

import { NavUserMenuContent } from "./NavUserMenuContent";

function NavUserTrigger() {
  return (
    <MenuTrigger
      variant="primary"
      size="icon"
      elevation="sm"
      aria-label="Account menu"
      data-testid="nav-user-menu"
    >
      <CircleUserIcon />
    </MenuTrigger>
  );
}

/** Auth-aware navbar component. Trigger is always stable; auth state resolves inside the dropdown. */
export function NavUser() {
  const { isAuthenticated, isLoading, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  const handleSignOut = () => {
    signOut();
    router.replace("/signin");
  };

  return (
    <Menu>
      <NavUserTrigger />
      <MenuContent className="w-56 p-2" offset="lg">
        <Stack className="gap-1">
          <NavUserMenuContent
            isAuthenticated={isAuthenticated}
            isLoading={isLoading}
            user={user}
            onSignOut={handleSignOut}
          />
        </Stack>
      </MenuContent>
    </Menu>
  );
}
