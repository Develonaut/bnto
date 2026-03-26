"use client";

import { useRouter } from "next/navigation";
import { core } from "@bnto/core";
import { toggleSignInMode } from "./toggleSignInMode";
import { handleSignInSubmit } from "./handleSignInSubmit";
import { useSignInFormFields } from "./useSignInFormFields";
import { useSignInMode } from "./useSignInMode";

export function useSignInForm(defaultMode?: "signin" | "signup") {
  const { email: signInEmail } = core.auth.useSignIn();
  const { email: signUpEmail } = core.auth.useSignUp();
  const auth = core.auth.useAuth();
  const router = useRouter();
  const { isSignUp, setMode, setUserToggled } = useSignInMode(defaultMode, auth.hasAccount);
  const fields = useSignInFormFields(auth.user?.email ?? "");

  function toggleMode() {
    const result = toggleSignInMode(isSignUp, auth.user?.email);
    setMode(result.nextMode);
    setUserToggled(true);
    fields.setError("");
    fields.setEmail(result.nextEmail);
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    fields.setError("");
    fields.setLoading(true);
    try {
      await handleSignInSubmit({ isSignUp, ...fields, signUpEmail, signInEmail });
      router.replace("/");
    } catch {
      fields.setError(
        isSignUp
          ? "Could not create account. Try a different email."
          : "Invalid email or password.",
      );
    } finally {
      fields.setLoading(false);
    }
  }

  return { isSignUp, ...fields, toggleMode, handleSubmit };
}
