import Link from "next/link";

import { ArrowLeftIcon } from "@/components/ui/icons";

import { Navbar } from "@/components/blocks/Navbar";
import { Footer } from "@/components/blocks/Footer";
import { AppShell } from "@/components/ui/AppShell";
import { Button } from "@/components/ui/Button";
import { GITHUB_URL } from "@/lib/copy";

/**
 * Root 404 page — self-contained with Navbar + Footer.
 *
 * In production (Vercel), `dynamicParams = false` 404s render inside the
 * (app) layout automatically. In dev mode, Next.js bypasses the route
 * group layout and renders this root not-found directly. Including the
 * full shell here ensures consistent rendering in both environments.
 */
export default function NotFound() {
  return (
    <>
      <AppShell.Header>
        <Navbar />
      </AppShell.Header>
      <AppShell.Main>
        <AppShell.Content>
          <div className="flex min-h-[60vh] flex-col items-center justify-center text-center">
            <h1 className="from-foreground to-foreground/70 relative mb-6 bg-linear-to-br bg-clip-text py-2 text-5xl font-bold text-transparent sm:text-6xl lg:text-7xl">
              Page Not Found
            </h1>

            <p className="text-muted-foreground mb-10 text-xl">
              Sorry, we couldn&apos;t find the page you&apos;re looking for. The
              page might have been removed or the URL might be incorrect.
            </p>

            <div className="flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
              <Button asChild className="group min-w-[200px] gap-2">
                <Link href="/">
                  <ArrowLeftIcon className="size-5 transition-transform group-hover:-translate-x-1" />
                  Back to Home
                </Link>
              </Button>
              <Button
                asChild
                variant="outline"
                size="md"
                className="min-w-[200px]"
              >
                <Link href={`${GITHUB_URL}/issues`}>Report Issue</Link>
              </Button>
            </div>
          </div>
        </AppShell.Content>
      </AppShell.Main>
      <Footer />
    </>
  );
}
