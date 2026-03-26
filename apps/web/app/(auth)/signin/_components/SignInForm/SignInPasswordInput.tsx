"use client";

import type { ChangeEvent } from "react";
import { PasswordInput } from "@bnto/ui";

interface SignInPasswordInputProps {
  isSignUp: boolean;
  value: string;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}

export function SignInPasswordInput({ isSignUp, value, onChange }: SignInPasswordInputProps) {
  return (
    <PasswordInput
      id="password"
      name="password"
      placeholder="Enter your password"
      value={value}
      onChange={onChange}
      required
      minLength={8}
      autoComplete={isSignUp ? "new-password" : "current-password"}
      data-testid="auth-password-input"
    />
  );
}
