"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { useSignIn, useSignUp } from "@bnto/auth";
import { Button } from "@bnto/ui/button";
import { Input } from "@bnto/ui/input";

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
    <div className="flex min-h-screen flex-col items-center justify-center bg-background p-4 text-foreground">
      <div className="w-full max-w-sm space-y-6">
        {/* Logo */}
        <div className="text-center">
          <span className="font-display text-2xl font-bold">bnto</span>
        </div>

        {/* Card */}
        <div className="rounded-xl bg-card p-6 shadow-lg">
          <div className="space-y-2 text-center">
            <h1 className="font-display text-xl font-semibold tracking-tight">
              {isSignUp ? "Create an account" : "Sign in to bnto"}
            </h1>
            <p className="text-sm text-muted-foreground">
              {isSignUp
                ? "Enter your details to get started"
                : "Enter your credentials to continue"}
            </p>
          </div>

          <form onSubmit={handleSubmit} className="mt-6 space-y-4">
            {isSignUp && (
              <div className="space-y-2">
                <label
                  htmlFor="name"
                  className="text-sm font-medium leading-none"
                >
                  Name
                </label>
                <Input
                  id="name"
                  type="text"
                  placeholder="Your name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  required
                />
              </div>
            )}

            <div className="space-y-2">
              <label
                htmlFor="email"
                className="text-sm font-medium leading-none"
              >
                Email
              </label>
              <Input
                id="email"
                type="email"
                placeholder="you@example.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                autoComplete="email"
              />
            </div>

            <div className="space-y-2">
              <label
                htmlFor="password"
                className="text-sm font-medium leading-none"
              >
                Password
              </label>
              <Input
                id="password"
                type="password"
                placeholder="Password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                minLength={8}
                autoComplete={isSignUp ? "new-password" : "current-password"}
              />
            </div>

            {error && (
              <p className="text-sm text-destructive" role="alert">
                {error}
              </p>
            )}

            <Button type="submit" disabled={loading} className="w-full">
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

        <p className="text-center text-sm text-muted-foreground">
          {isSignUp ? "Already have an account?" : "No account?"}{" "}
          <button
            type="button"
            onClick={toggleMode}
            className="font-medium text-foreground underline underline-offset-4 hover:text-foreground/80"
          >
            {isSignUp ? "Sign in" : "Sign up"}
          </button>
        </p>
      </div>
    </div>
  );
}
