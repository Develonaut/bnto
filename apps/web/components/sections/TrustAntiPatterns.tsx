import { CheckIcon, FadeIn, IconBadge, Row, Stack } from "@bnto/ui";

const ANTI_PATTERNS = [
  "Signup required",
  "File size limits",
  "Daily usage caps",
  "Watermarks on output",
  "Quality reduction",
  "\u201CUpgrade to continue\u201D",
];

interface TrustAntiPatternsProps {
  /** Extra delay in ms before the stagger cascade starts (e.g. to wait for a dormant card spring). */
  baseDelay?: number;
}

export function TrustAntiPatterns({ baseDelay = 0 }: TrustAntiPatternsProps) {
  return (
    <Stack className="gap-3">
      {ANTI_PATTERNS.map((item, i) => (
        <FadeIn
          key={item}
          index={i}
          style={
            baseDelay > 0
              ? {
                  animationDelay: `calc(${baseDelay}ms + var(--stagger-index, 0) * var(--stagger-interval, 60ms))`,
                }
              : undefined
          }
        >
          <Row className="gap-3">
            <IconBadge variant="destructive" size="sm">
              <CheckIcon className="size-3.5" />
            </IconBadge>
            <span className="text-sm text-muted-foreground line-through decoration-muted-foreground/40">
              {item}
            </span>
          </Row>
        </FadeIn>
      ))}
    </Stack>
  );
}
