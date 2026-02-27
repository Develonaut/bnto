"use client";

import {
  CheckIcon,
  GithubIcon,
} from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
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
      <div className="flex w-full items-center justify-center">
        <Card elevation="md" className="w-full max-w-sm p-6">
          <div className="flex flex-col gap-5">
            <div className="flex flex-col gap-3">
              {ANTI_PATTERNS.map((item, i) => (
                <Animate.SlideUp key={item} index={i} distance={8} easing="spring-bouncy">
                  <div className="flex items-center gap-3">
                    <div className="flex size-6 shrink-0 items-center justify-center rounded-full bg-destructive/10">
                      <CheckIcon className="size-3.5 text-destructive" />
                    </div>
                    <span className="text-sm text-muted-foreground line-through decoration-muted-foreground/40">
                      {item}
                    </span>
                  </div>
                </Animate.SlideUp>
              ))}
            </div>

            <div className="border-t border-border pt-4">
              <div className="flex items-center justify-between">
                <Text size="xs" mono color="muted" className="uppercase tracking-wider">
                  MIT Licensed &middot; Open Source
                </Text>
                <Button
                  variant="outline"
                  size="sm"
                  href={GITHUB_URL}
                  target="_blank"
                  rel="noopener noreferrer"
                  elevation="sm"
                >
                  <GithubIcon className="size-3.5" />
                  GitHub
                </Button>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </Animate.ScaleIn>
  );
}
