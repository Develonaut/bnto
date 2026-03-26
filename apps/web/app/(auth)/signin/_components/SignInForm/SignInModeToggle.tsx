"use client";

import { Text } from "@bnto/ui";

interface SignInModeToggleProps {
  isSignUp: boolean;
  onToggle: () => void;
}

/** "Already have an account? Sign in" / "Don't have an account? Sign up" toggle. */
export function SignInModeToggle({ isSignUp, onToggle }: SignInModeToggleProps) {
  return (
    <Text size="sm" color="muted" className="text-center">
      {isSignUp ? "Already have an account?" : "Don't have an account?"}{" "}
      <button
        type="button"
        onClick={onToggle}
        data-testid="auth-mode-toggle"
        className="text-primary font-medium"
      >
        {isSignUp ? "Sign in" : "Sign up"}
      </button>
    </Text>
  );
}
