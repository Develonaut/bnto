"use client";

import type { ChangeEvent } from "react";
import { Input } from "@bnto/ui";

interface SignInEmailInputProps {
  value: string;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}

export function SignInEmailInput({ value, onChange }: SignInEmailInputProps) {
  return (
    <Input
      id="email"
      name="email"
      type="email"
      placeholder="Enter your email"
      value={value}
      onChange={onChange}
      required
      autoComplete="email"
      data-testid="auth-email-input"
    />
  );
}
