"use client";

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

  function handleSignOut() {
    signOut();
    onClose();
    router.replace("/signin");
  }

  if (!isAuthenticated || !user?.email) {
    return (
      <Button variant="secondary" href="/signin" onClick={onClose} data-testid="mobile-sign-in">
        Sign In
      </Button>
    );
  }

  return <MobileNavAuthProfile name={user.name} email={user.email} onSignOut={handleSignOut} />;
}
