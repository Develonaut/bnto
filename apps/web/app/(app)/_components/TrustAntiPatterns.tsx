import { CheckIcon, IconBadge, Row, SlideUp, Stack } from "@bnto/ui";

const ANTI_PATTERNS = [
  "Signup required",
  "File size limits",
  "Daily usage caps",
  "Watermarks on output",
  "Quality reduction",
  "\u201CUpgrade to continue\u201D",
];

export function TrustAntiPatterns() {
  return (
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
  );
}
