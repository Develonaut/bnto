"use client";

import Link from "next/link";
import type { AuthUser } from "@bnto/core";

import { LogInIcon, LogOutIcon, MenuSeparator, MenuItem, Skeleton, Text } from "@bnto/ui";

interface NavUserMenuContentProps {
  isAuthenticated: boolean;
  isLoading: boolean;
  user: AuthUser | null;
  onSignOut: () => void;
}

export function NavUserMenuContent({
  isAuthenticated,
  isLoading,
  user,
  onSignOut,
}: NavUserMenuContentProps) {
  if (isLoading) return <NavUserSkeleton />;
  if (!isAuthenticated) return <NavUserSignIn />;
  return <NavUserProfile user={user} onSignOut={onSignOut} />;
}

export function NavUserSkeleton() {
  return (
    <div className="px-3 py-2">
      <Skeleton className="h-4 w-24" />
      <Skeleton className="mt-1.5 h-3 w-36" />
    </div>
  );
}

export function NavUserSignIn() {
  return (
    <MenuItem asChild data-testid="nav-sign-in">
      <Link href="/signin">
        <LogInIcon />
        Sign in
      </Link>
    </MenuItem>
  );
}

export function NavUserProfile({
  user,
  onSignOut,
}: {
  user: AuthUser | null;
  onSignOut: () => void;
}) {
  return (
    <>
      <div className="px-3 py-2">
        {user?.name && (
          <Text size="sm" className="font-medium">
            {user.name}
          </Text>
        )}
        <Text size="xs" color="muted" data-testid="nav-user-email">
          {user?.email}
        </Text>
      </div>
      <MenuSeparator />
      <MenuItem onClick={onSignOut} data-testid="nav-sign-out">
        <LogOutIcon />
        Sign out
      </MenuItem>
    </>
  );
}
