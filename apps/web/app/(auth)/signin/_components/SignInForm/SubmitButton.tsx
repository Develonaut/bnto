"use client";

import { Button, LoaderIcon } from "@bnto/ui";

interface SubmitButtonProps {
  loading: boolean;
  isSignUp: boolean;
}

export function SubmitButton({ loading, isSignUp }: SubmitButtonProps) {
  const label = loading
    ? isSignUp
      ? "Creating account..."
      : "Signing in..."
    : isSignUp
      ? "Create account"
      : "Sign in";

  return (
    <Button type="submit" fullWidth disabled={loading} data-testid="auth-submit">
      {loading && <LoaderIcon className="size-4 motion-safe:animate-spin" />}
      {label}
    </Button>
  );
}
