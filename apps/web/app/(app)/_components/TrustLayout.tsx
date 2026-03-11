import { CheckIcon } from "@bnto/ui";

import { ScaleIn, SlideUp, Card, Center, IconBadge, Row, Stack, Text } from "@bnto/ui";

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
    <ScaleIn from={0.9} easing="spring-bouncy">
      <Center className="w-full">
        <Card className="w-full max-w-sm p-6">
          <Stack className="gap-5">
            <Stack className="gap-3">
              {ANTI_PATTERNS.map((item, i) => (
                <SlideUp key={item} index={i} distance={8} easing="spring-bouncy">
                  <Row className="gap-3">
                    <IconBadge variant="destructive" size="sm">
                      <CheckIcon className="size-3.5" />
                    </IconBadge>
                    <span className="text-sm text-muted-foreground line-through decoration-muted-foreground/40">
                      {item}
                    </span>
                  </Row>
                </SlideUp>
              ))}
            </Stack>

            <div className="border-t border-border pt-4">
              <Text size="xs" mono color="muted" className="uppercase tracking-wider">
                Free forever. No limits.
              </Text>
            </div>
          </Stack>
        </Card>
      </Center>
    </ScaleIn>
  );
}
