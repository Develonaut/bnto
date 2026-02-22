import Link from "next/link";

/**
 * Landing page for unauthenticated users on /.
 *
 * Full-page with its own chrome — no app shell wrapping.
 */
export function LandingPage() {
  return (
    <main className="flex min-h-svh flex-col items-center justify-center gap-6 px-4">
      <h1 className="text-4xl font-bold tracking-tight">bnto</h1>
      <p className="max-w-md text-center text-muted-foreground">
        The one place small teams go to get things done — compress images,
        clean a CSV, rename files, call an API — without the overhead of a
        platform or the fragility of a script.
      </p>
      <Link
        href="/signin"
        className="inline-flex h-10 items-center justify-center rounded-md bg-primary px-8 text-sm font-medium text-primary-foreground shadow-sm hover:bg-primary/90"
      >
        Get started
      </Link>
    </main>
  );
}
