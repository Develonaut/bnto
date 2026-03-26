import type { CSSProperties } from "react";
import { Heading, Stack, Text } from "@bnto/ui";
import { NavButton } from "@/components/blocks/NavButton";

interface SignInHeaderProps {
  isSignUp: boolean;
}

export function SignInHeader({ isSignUp }: SignInHeaderProps) {
  return (
    <Stack gap="sm" align="center" className="text-center">
      <NavButton
        href="/"
        style={
          { "--face-bg": "var(--background)", "--face-fg": "var(--foreground)" } as CSSProperties
        }
        className="w-fit text-xl font-display font-black tracking-tighter"
      >
        bnto
      </NavButton>
      <Heading level={1} size="sm" data-testid="auth-heading">
        {isSignUp ? "Create an account" : "Welcome back"}
      </Heading>
      <Text color="muted">
        {isSignUp ? "Enter your details to get started" : "Please enter your details."}
      </Text>
    </Stack>
  );
}
