import Link from "next/link";
import { Button } from "@bnto/ui/button";

/**
 * Branded 404 page -- shown for unknown routes.
 *
 * Clean and minimal with navigation back to home.
 */
export default function NotFound() {
  return (
    <div className="flex min-h-svh flex-col items-center justify-center gap-4 p-4">
      <h1 className="text-4xl font-bold tracking-tight text-foreground">
        404
      </h1>
      <p className="text-muted-foreground">This page doesn&apos;t exist.</p>
      <Button asChild>
        <Link href="/">Back to home</Link>
      </Button>
    </div>
  );
}
