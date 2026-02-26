"use client";

import { AppShell } from "@/components/ui/AppShell";

import Privacy from "./privacy.mdx";

export default function PrivacyPage() {
  return (
    <AppShell.Content>
      <article className="prose prose-lg dark:prose-invert mx-auto max-w-2xl">
        <Privacy />
      </article>
    </AppShell.Content>
  );
}
