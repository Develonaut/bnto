"use client";

import { Button, Card, GithubIcon, Row, Stagger, Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";

import { TrustAntiPatterns } from "./TrustAntiPatterns";
import { useStepInView } from "./useStepInView";

export function TrustLayout() {
  const [inView, ref] = useStepInView(0.3);

  return (
    <Card ref={ref} dormant={!inView} elevation="lg" className="w-full p-6">
      <Stack className="gap-5">
        <div className="grid items-center gap-6 sm:grid-cols-[auto_1fr]">
          {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
          <img
            src="/mascots/sumo-sushi.svg"
            alt=""
            width={260}
            height={260}
            className="mx-auto shrink-0"
            aria-hidden
          />
          <Stagger>
            <TrustAntiPatterns baseDelay={250} />
          </Stagger>
        </div>
        <div className="border-t border-border pt-4">
          <Row justify="between">
            <Text size="xs" mono color="muted" className="uppercase tracking-wider">
              MIT Licensed &middot; Open Source
            </Text>
            <Button variant="secondary" href={GITHUB_URL} target="_blank" rel="noopener noreferrer">
              <GithubIcon className="size-3.5" />
              GitHub
            </Button>
          </Row>
        </div>
      </Stack>
    </Card>
  );
}
