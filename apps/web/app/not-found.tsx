import { ArrowLeftIcon } from "@/components/ui/icons";

import { Navbar } from "@/components/blocks/Navbar";
import { Footer } from "@/components/blocks/Footer";
import { AppShell } from "@/components/ui/AppShell";
import { Button } from "@/components/ui/Button";
import { Heading } from "@/components/ui/Heading";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
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
          <Stack
            align="center"
            justify="center"
            className="min-h-[60vh] text-center"
          >
            <Stack gap="md" align="center" className="max-w-lg">
              <Text
                size="sm"
                mono
                color="muted"
                className="uppercase tracking-wider"
              >
                404
              </Text>
              <Heading level={1}>Page Not Found</Heading>
              <Text color="muted" size="lg" balance>
                Sorry, we couldn&apos;t find the page you&apos;re looking for.
                The page might have been removed or the URL might be incorrect.
              </Text>
            </Stack>

            <Row gap="sm" className="pt-4">
              <Button href="/" className="group gap-2">
                <ArrowLeftIcon className="size-4 transition-transform group-hover:-translate-x-1" />
                Back to Home
              </Button>
              <Button
                variant="outline"
                href={`${GITHUB_URL}/issues`}
                target="_blank"
                rel="noopener noreferrer"
              >
                Report Issue
              </Button>
            </Row>
          </Stack>
        </AppShell.Content>
      </AppShell.Main>
      <Footer />
    </>
  );
}
