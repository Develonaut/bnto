"use client";

import { Button, LogOutIcon, Stack, Text } from "@bnto/ui";

interface MobileNavAuthProfileProps {
  name?: string;
  email?: string;
  onSignOut: () => void;
}

/** Authenticated user profile + sign out button in the mobile nav. */
export function MobileNavAuthProfile({ name, email, onSignOut }: MobileNavAuthProfileProps) {
  return (
    <Stack className="gap-3">
      {(name || email) && (
        <div>
          {name && (
            <Text size="sm" className="font-medium text-primary-foreground">
              {name}
            </Text>
          )}
          {email && (
            <Text size="xs" className="text-primary-foreground/60">
              {email}
            </Text>
          )}
        </div>
      )}
      <Button variant="outline" onClick={onSignOut} data-testid="mobile-sign-out">
        <LogOutIcon />
        Sign out
      </Button>
    </Stack>
  );
}
