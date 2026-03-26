"use client";

import { Container, Stack } from "@bnto/ui";
import { useSignInForm } from "./useSignInForm";
import { SignInHeader } from "./SignInHeader";
import { SignInFormCard } from "./SignInFormCard";
import { SignInModeToggle } from "./SignInModeToggle";

interface SignInFormProps {
  defaultMode?: "signin" | "signup";
}

export function SignInFormRoot({ defaultMode }: SignInFormProps) {
  const form = useSignInForm(defaultMode);

  return (
    <section className="flex flex-1 items-center justify-center py-8">
      <Container>
        <Stack gap="md" className="mx-auto w-full max-w-sm">
          <SignInHeader isSignUp={form.isSignUp} />
          <SignInFormCard
            isSignUp={form.isSignUp}
            name={form.name}
            email={form.email}
            password={form.password}
            error={form.error}
            loading={form.loading}
            onNameChange={form.onNameChange}
            onEmailChange={form.onEmailChange}
            onPasswordChange={form.onPasswordChange}
            onSubmit={form.handleSubmit}
          />
          <SignInModeToggle isSignUp={form.isSignUp} onToggle={form.toggleMode} />
        </Stack>
      </Container>
    </section>
  );
}
