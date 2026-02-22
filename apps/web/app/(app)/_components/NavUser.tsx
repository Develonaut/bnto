"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { LogOutIcon, UserIcon } from "@bnto/ui";
import { useCurrentUser, useIsAuthenticated, useSignOut } from "@bnto/core";
import { Avatar } from "@bnto/ui/avatar";
import { Button } from "@bnto/ui/button";
import { DropdownMenu } from "@bnto/ui/dropdown-menu";

/** Initials from a full name (e.g. "Jane Doe" -> "JD"). */
function initials(name: string) {
  return name
    .split(" ")
    .map((part) => part[0])
    .join("")
    .toUpperCase();
}

/**
 * Self-fetching user dropdown, or "Sign in" link for anonymous users.
 *
 * Authenticated: avatar + name dropdown with account and sign-out.
 * Unauthenticated: simple "Sign in" link button.
 */
export function NavUser() {
  const isAuthenticated = useIsAuthenticated();

  if (!isAuthenticated) {
    return (
      <Button variant="ghost" asChild>
        <Link href="/signin">Sign in</Link>
      </Button>
    );
  }

  return <AuthenticatedNavUser />;
}

function AuthenticatedNavUser() {
  const router = useRouter();
  const { data: user } = useCurrentUser();
  const signOut = useSignOut();

  const displayName = user?.name ?? "User";

  function handleSignOut() {
    signOut();
    router.replace("/signin");
  }

  return (
    <DropdownMenu>
      <DropdownMenu.Trigger asChild>
        <Button variant="ghost" className="gap-2 px-2">
          <Avatar className="size-8">
            <Avatar.Image src={user?.image ?? undefined} alt={displayName} />
            <Avatar.Fallback>{initials(displayName)}</Avatar.Fallback>
          </Avatar>
          <span className="hidden text-sm font-medium md:inline">
            {displayName}
          </span>
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end" className="w-56">
        <DropdownMenu.Label className="font-normal">
          <div className="flex flex-col space-y-1">
            <p className="text-sm font-medium">{displayName}</p>
            {user?.email && (
              <p className="text-xs text-muted-foreground">{user.email}</p>
            )}
          </div>
        </DropdownMenu.Label>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onSelect={() => router.push("/settings")}>
          <UserIcon className="mr-2 size-4" />
          Account
        </DropdownMenu.Item>
        <DropdownMenu.Separator />
        <DropdownMenu.Item onSelect={handleSignOut}>
          <LogOutIcon className="mr-2 size-4" />
          Sign out
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu>
  );
}
