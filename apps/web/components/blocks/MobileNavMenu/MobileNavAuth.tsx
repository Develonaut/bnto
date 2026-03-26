"use client";

import { useCallback } from "react";

import { core } from "@bnto/core";
import { useRouter } from "next/navigation";
import { Button } from "@bnto/ui";
import { MobileNavAuthProfile } from "./MobileNavAuthProfile";

interface MobileNavAuthProps {
  onClose: () => void;
}

export function MobileNavAuth({ onClose }: MobileNavAuthProps) {
  const { isAuthenticated, user } = core.auth.useAuth();
  const signOut = core.auth.useSignOut();
  const router = useRouter();

  const handleSignOut = useCallback(() => {
    signOut();
    onClose();
    router.replace("/signin");
  }, [signOut, onClose, router]);

  if (!isAuthenticated || !user?.email) {
    return (
      <Button variant="secondary" href="/signin" onClick={onClose} data-testid="mobile-sign-in">
        Sign In
      </Button>
    );
  }

  return <MobileNavAuthProfile name={user.name} email={user.email} onSignOut={handleSignOut} />;
}
