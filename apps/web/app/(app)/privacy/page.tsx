"use client";

import { AppShellContent } from "@bnto/ui";

import Privacy from "./privacy.mdx";

export default function PrivacyPage() {
  return (
    <AppShellContent>
      <article className="prose prose-neutral dark:prose-invert mx-auto max-w-2xl [&_hr]:my-8">
        <Privacy />
      </article>
    </AppShellContent>
  );
}
