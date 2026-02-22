import Link from "next/link";
import { Button } from "@bnto/ui/button";
import { Background } from "@bnto/ui/background";

/**
 * Branded 404 page -- shown for unknown routes.
 *
 * Gradient background with display font heading and nav back home.
 */
export default function NotFound() {
  return (
    <Background variant="top" className="flex min-h-svh flex-col items-center justify-center gap-4 p-4">
      <h1 className="font-display text-6xl font-bold tracking-tight text-foreground">
        404
      </h1>
      <p className="text-muted-foreground">This page doesn&apos;t exist.</p>
      <Button asChild className="mt-2 rounded-full">
        <Link href="/">Back to home</Link>
      </Button>
    </Background>
  );
}
