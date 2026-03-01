"use client";

import {
  CheckIcon,
  GithubIcon,
} from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Center } from "@/components/ui/Center";
import { IconBadge } from "@/components/ui/IconBadge";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { GITHUB_URL } from "@/lib/copy";

/* ── Data ────────────────────────────────────────────────────── */

const ANTI_PATTERNS = [
  "Signup required",
  "File size limits",
  "Daily usage caps",
  "Watermarks on output",
  "Quality reduction",
  "\u201CUpgrade to continue\u201D",
];

/* ── Trust layout ────────────────────────────────────────────── */

export function TrustLayout() {
  return (
    <Animate.ScaleIn from={0.9} easing="spring-bouncy">
      <Center className="w-full">
        <Card elevation="md" className="w-full max-w-sm p-6">
          <Stack className="gap-5">
            <Stack className="gap-3">
              {ANTI_PATTERNS.map((item, i) => (
                <Animate.SlideUp key={item} index={i} distance={8} easing="spring-bouncy">
                  <Row className="gap-3">
                    <IconBadge variant="destructive" size="sm">
                      <CheckIcon className="size-3.5" />
                    </IconBadge>
                    <span className="text-sm text-muted-foreground line-through decoration-muted-foreground/40">
                      {item}
                    </span>
                  </Row>
                </Animate.SlideUp>
              ))}
            </Stack>

            <div className="border-t border-border pt-4">
              <Row justify="between">
                <Text size="xs" mono color="muted" className="uppercase tracking-wider">
                  MIT Licensed &middot; Open Source
                </Text>
                <Button
                  variant="outline"
                  size="md"
                  href={GITHUB_URL}
                  target="_blank"
                  rel="noopener noreferrer"
                  elevation="sm"
                >
                  <GithubIcon className="size-3.5" />
                  GitHub
                </Button>
              </Row>
            </div>
          </Stack>
        </Card>
      </Center>
    </Animate.ScaleIn>
  );
}
