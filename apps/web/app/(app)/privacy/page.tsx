"use client";

import { AppShell } from "@/components/ui/AppShell";

import Privacy from "./privacy.mdx";

export default function PrivacyPage() {
  return (
    <AppShell.Content>
      <article className="prose prose-neutral dark:prose-invert mx-auto max-w-2xl [&_hr]:my-8">
        <Privacy />
      </article>
    </AppShell.Content>
  );
}
