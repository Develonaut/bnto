"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { useSignIn, useSignUp } from "@bnto/auth";
import { Background } from "#components/Background";
import { Button } from "#components/ui/button";
import { Container } from "#components/ui/Container";
import { Heading } from "#components/ui/Heading";
import { Text } from "#components/ui/Text";
import { Stack } from "#components/ui/Stack";
import { Input } from "#components/ui/input";

type Mode = "signin" | "signup";

interface SignInFormProps {
  defaultMode?: Mode;
}

export function SignInForm({ defaultMode = "signin" }: SignInFormProps) {
  const signIn = useSignIn();
  const signUp = useSignUp();
  const router = useRouter();

  const [mode, setMode] = useState<Mode>(defaultMode);
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

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
        await signUp.email({ name, email, password });
      } else {
        await signIn.email({ email, password });
      }
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
    <Background>
      <section className="py-28 lg:pt-44 lg:pb-32">
        <Container>
          <Stack gap="md" className="mx-auto w-full max-w-sm">
            <Stack gap="sm" align="center" className="text-center">
              <Link href="/" className="mb-4">
                <span className="font-display text-2xl font-bold">bnto</span>
              </Link>
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
                    type="text"
                    placeholder="Your name"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                  />
                )}
                <Input
                  type="email"
                  placeholder="Enter your email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  autoComplete="email"
                />
                <Input
                  type="password"
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
    </Background>
  );
}
