"use client";

import { useEffect, useState, type FormEvent } from "react";
import Link from "next/link";
import { Button, Input } from "@bnto/ui";

export default function SplashPage() {
  const [unlocked, setUnlocked] = useState(false);
  const [guess, setGuess] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (document.cookie.includes("bnto-access=granted")) {
      setUnlocked(true);
    }
  }, []);

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    if (!guess.trim() || loading) return;

    setLoading(true);
    setError("");

    const res = await fetch("/api/verify-passphrase", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ passphrase: guess }),
    });

    const data = await res.json();
    setLoading(false);

    if (data.valid) {
      setUnlocked(true);
    } else {
      setError("Nope. Try again.");
      setGuess("");
    }
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-background text-foreground">
      <h1 className="text-5xl font-bold tracking-tight">Bnto</h1>
      <p className="mt-4 text-lg text-muted-foreground">
        Workflow automation, simplified.
      </p>

      {unlocked ? (
        <div className="mt-10 flex gap-4">
          <Button asChild>
            <Link href="/sign-in">Sign In</Link>
          </Button>
          <Button asChild variant="outline">
            <Link href="/sign-up">Sign Up</Link>
          </Button>
        </div>
      ) : (
        <form onSubmit={handleSubmit} className="mt-10 w-full max-w-md">
          <div className="flex items-center gap-3 rounded-lg border border-border bg-card px-4 py-3 font-mono text-sm">
            <span className="select-none text-muted-foreground">&gt;</span>
            <Input
              value={guess}
              onChange={(e) => setGuess(e.target.value)}
              placeholder="Enter passphrase"
              disabled={loading}
              className="border-0 bg-transparent p-0 font-mono shadow-none focus-visible:ring-0"
            />
          </div>
          {error && (
            <p className="mt-2 text-center text-sm text-destructive">
              {error}
            </p>
          )}
        </form>
      )}
    </main>
  );
}
