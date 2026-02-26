import Link from "next/link";

import { ArrowLeftIcon } from "@/components/ui/icons";

import { AppShell } from "@/components/ui/AppShell";
import { Button } from "@/components/ui/Button";
import { GITHUB_URL } from "@/lib/copy";

export default function NotFound() {
  return (
    <AppShell.Content>
      <div className="flex min-h-[60vh] flex-col items-center justify-center text-center">
        <h1 className="from-foreground to-foreground/70 relative mb-6 bg-linear-to-br bg-clip-text py-2 text-5xl font-bold text-transparent sm:text-6xl lg:text-7xl">
          Page Not Found
        </h1>

        <p className="text-muted-foreground mb-10 text-xl">
          Sorry, we couldn&apos;t find the page you&apos;re looking for. The page might
          have been removed or the URL might be incorrect.
        </p>

        <div className="flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
          <Button asChild size="lg" className="group min-w-[200px] gap-2">
            <Link href="/">
              <ArrowLeftIcon className="size-5 transition-transform group-hover:-translate-x-1" />
              Back to Home
            </Link>
          </Button>
          <Button
            asChild
            variant="outline"
            size="lg"
            className="min-w-[200px]"
          >
            <Link href={`${GITHUB_URL}/issues`}>Report Issue</Link>
          </Button>
        </div>
      </div>
    </AppShell.Content>
  );
}
