"use client";

import type { ChangeEvent } from "react";
import { Input } from "@bnto/ui";
import { SignInEmailInput } from "./SignInEmailInput";
import { SignInPasswordInput } from "./SignInPasswordInput";

interface SignInFormInputsProps {
  isSignUp: boolean;
  name: string;
  email: string;
  password: string;
  onNameChange: (e: ChangeEvent<HTMLInputElement>) => void;
  onEmailChange: (e: ChangeEvent<HTMLInputElement>) => void;
  onPasswordChange: (e: ChangeEvent<HTMLInputElement>) => void;
}

export function SignInFormInputs({
  isSignUp,
  name,
  email,
  password,
  onNameChange,
  onEmailChange,
  onPasswordChange,
}: SignInFormInputsProps) {
  return (
    <>
      {isSignUp && (
        <Input
          id="name"
          name="name"
          type="text"
          placeholder="Your name"
          value={name}
          onChange={onNameChange}
          required
          autoComplete="name"
          data-testid="auth-name-input"
        />
      )}
      <SignInEmailInput value={email} onChange={onEmailChange} />
      <SignInPasswordInput isSignUp={isSignUp} value={password} onChange={onPasswordChange} />
    </>
  );
}
