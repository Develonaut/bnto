"use client";

import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";

interface UpgradePromptProps {
  /** The bnto slug -- used as return parameter after signup. */
  slug: string;
  /** The reason the prompt is shown. */
  reason: "quota" | "save" | "history";
}

const COPY: Record<UpgradePromptProps["reason"], { heading: string; body: string }> = {
  quota: {
    heading: "You've used all your guest runs",
    body: "Sign up for a free account to keep running workflows.",
  },
  save: {
    heading: "Sign up to save your work",
    body: "Create a free account to save workflows and access them anytime.",
  },
  history: {
    heading: "Sign up to view history",
    body: "Create a free account to see your past executions and results.",
  },
};

/**
 * Inline prompt shown to anonymous users when they hit a conversion
 * touchpoint -- quota exhausted, attempting to save, or viewing history.
 *
 * Navigates to /signin with a return parameter so the user can
 * continue where they left off after creating an account.
 */
export function UpgradePrompt({ slug, reason }: UpgradePromptProps) {
  const router = useRouter();
  const { heading, body } = COPY[reason];

  function handleSignUp() {
    router.push(`/signin?return=/${encodeURIComponent(slug)}`);
  }

  return (
    <div className="rounded-lg border border-border bg-card p-6 text-center" data-testid="upgrade-prompt">
      <h2 className="text-lg font-semibold text-foreground">{heading}</h2>
      <p className="mt-2 text-sm text-muted-foreground">{body}</p>
      <Button onClick={handleSignUp} className="mt-4">
        Sign up free
      </Button>
    </div>
  );
}
