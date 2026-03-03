"use client";

import type { CSSProperties } from "react";
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { core } from "@bnto/core";
import { NavButton } from "@/components/blocks/NavButton";
import { Button } from "@/components/ui/Button";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { Input } from "@/components/ui/Input";
import { PasswordInput } from "@/components/ui/PasswordInput";
import { LoaderIcon } from "@/components/ui/icons";

type Mode = "signin" | "signup";

const HAS_ACCOUNT_COOKIE = "bnto-has-account";

function hasCookie(name: string) {
  return typeof document !== "undefined" && document.cookie.includes(name);
}

function setHasAccountCookie() {
  document.cookie = `${HAS_ACCOUNT_COOKIE}=1; path=/; max-age=${60 * 60 * 24 * 365}; samesite=lax`;
}

interface SignInFormProps {
  defaultMode?: Mode;
}

export function SignInForm({ defaultMode }: SignInFormProps) {
  const { email: signInEmail } = core.auth.useSignIn();
  const { email: signUpEmail } = core.auth.useSignUp();
  const router = useRouter();

  // Default to signup for new visitors, signin for returning users.
  // The bnto-has-account cookie is set on successful auth and persists for 1 year.
  const resolvedDefault = defaultMode ?? (hasCookie(HAS_ACCOUNT_COOKIE) ? "signin" : "signup");
  const [mode, setMode] = useState<Mode>(resolvedDefault);
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  // Pre-fill email from localStorage for returning users
  useEffect(() => {
    const remembered = localStorage.getItem("bnto-email");
    if (remembered) setEmail(remembered);
  }, []);

  const isSignUp = mode === "signup";

  function toggleMode() {
    setMode(isSignUp ? "signin" : "signup");
    setError("");
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      if (isSignUp) {
        await signUpEmail({ name, email, password });
      } else {
        await signInEmail({ email, password });
      }
      setHasAccountCookie();
      localStorage.setItem("bnto-email", email);
      router.replace("/");
    } catch {
      setError(
        isSignUp
          ? "Could not create account. Try a different email."
          : "Invalid email or password.",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="py-20 lg:py-28">
      <Container>
          <Stack gap="md" className="mx-auto w-full max-w-sm">
            <Stack gap="sm" align="center" className="text-center">
              <NavButton
                href="/"
                style={{ "--face-bg": "var(--background)", "--face-fg": "var(--foreground)" } as CSSProperties}
                className="mb-4 text-xl font-display font-black tracking-tighter"
              >
                bnto
              </NavButton>
              <Heading level={1} size="sm">
                {isSignUp ? "Create an account" : "Welcome back"}
              </Heading>
              <Text color="muted">
                {isSignUp
                  ? "Enter your details to get started"
                  : "Please enter your details."}
              </Text>
            </Stack>

            <div className="rounded-xl border bg-card p-6 shadow-sm">
              <form onSubmit={handleSubmit} className="grid gap-4">
                {isSignUp && (
                  <Input
                    id="name"
                    name="name"
                    type="text"
                    placeholder="Your name"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                    autoComplete="name"
                  />
                )}
                <Input
                  id="email"
                  name="email"
                  type="email"
                  placeholder="Enter your email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  autoComplete="email"
                />
                <PasswordInput
                  id="password"
                  name="password"
                  placeholder="Enter your password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  minLength={8}
                  autoComplete={isSignUp ? "new-password" : "current-password"}
                />

                {error && (
                  <p className="text-sm text-destructive" role="alert">
                    {error}
                  </p>
                )}

                <Button type="submit" disabled={loading} className="mt-2 w-full">
                  {loading && (
                    <LoaderIcon className="size-4 motion-safe:animate-spin" />
                  )}
                  {loading
                    ? isSignUp
                      ? "Creating account..."
                      : "Signing in..."
                    : isSignUp
                      ? "Create account"
                      : "Sign in"}
                </Button>
              </form>
            </div>

            <Text size="sm" color="muted" className="text-center">
              {isSignUp ? "Already have an account?" : "Don't have an account?"}{" "}
              <button
                type="button"
                onClick={toggleMode}
                className="text-primary font-medium"
              >
                {isSignUp ? "Sign in" : "Sign up"}
              </button>
            </Text>
          </Stack>
      </Container>
    </section>
  );
}
