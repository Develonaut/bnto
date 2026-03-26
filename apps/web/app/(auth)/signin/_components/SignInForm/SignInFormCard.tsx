"use client";

import type { ChangeEvent } from "react";
import { SignInFormInputs } from "./SignInFormInputs";
import { SubmitButton } from "./SubmitButton";

interface SignInFormCardProps {
  isSignUp: boolean;
  name: string;
  email: string;
  password: string;
  error: string;
  loading: boolean;
  onNameChange: (e: ChangeEvent<HTMLInputElement>) => void;
  onEmailChange: (e: ChangeEvent<HTMLInputElement>) => void;
  onPasswordChange: (e: ChangeEvent<HTMLInputElement>) => void;
  onSubmit: (e: React.FormEvent) => void;
}

function FormError({ error }: { error: string }) {
  if (!error) return null;
  return (
    <p className="text-sm text-destructive" role="alert" data-testid="auth-error">
      {error}
    </p>
  );
}

export function SignInFormCard({
  isSignUp,
  name,
  email,
  password,
  error,
  loading,
  onNameChange,
  onEmailChange,
  onPasswordChange,
  onSubmit,
}: SignInFormCardProps) {
  return (
    <div className="rounded-xl border bg-card p-6 shadow-sm">
      <form onSubmit={onSubmit} className="grid gap-4">
        <SignInFormInputs
          isSignUp={isSignUp}
          name={name}
          email={email}
          password={password}
          onNameChange={onNameChange}
          onEmailChange={onEmailChange}
          onPasswordChange={onPasswordChange}
        />
        <FormError error={error} />
        <SubmitButton loading={loading} isSignUp={isSignUp} />
      </form>
    </div>
  );
}
