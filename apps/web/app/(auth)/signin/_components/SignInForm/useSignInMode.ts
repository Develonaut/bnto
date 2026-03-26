"use client";

import { useState } from "react";

type Mode = "signin" | "signup";

/** Manages the sign-in / sign-up mode toggle with auto-detection from auth state. */
export function useSignInMode(defaultMode: Mode | undefined, hasAccount: boolean) {
  const resolvedDefault = defaultMode ?? (hasAccount ? "signin" : "signup");
  const [mode, setMode] = useState<Mode>(resolvedDefault);
  const [userToggled, setUserToggled] = useState(false);

  const [prevHasAccount, setPrevHasAccount] = useState(hasAccount);
  if (hasAccount !== prevHasAccount) {
    setPrevHasAccount(hasAccount);
    if (!defaultMode && !userToggled && hasAccount) setMode("signin");
  }

  return { mode, setMode, setUserToggled, isSignUp: mode === "signup" };
}
