import { Heading, SlideUp, Stack, Text } from "@bnto/ui";

interface SectionHeaderProps {
  label?: string;
  title: string;
  subtitle?: string;
  mascot?: string;
  mascotHeight?: number;
}

export function SectionHeader({
  label,
  title,
  subtitle,
  mascot,
  mascotHeight = 72,
}: SectionHeaderProps) {
  return (
    <SlideUp>
      <Stack gap="md" className="items-center text-center">
        {mascot && (
          // eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed
          <img
            src={mascot}
            alt=""
            className="w-auto"
            style={{ height: mascotHeight }}
            aria-hidden
          />
        )}
        <Text size="sm" mono color="muted" className="uppercase tracking-wider">
          {label}
        </Text>
        <Heading level={2} size="xl">
          {title}
        </Heading>
        {subtitle && (
          <Text color="muted" leading="snug">
            {subtitle}
          </Text>
        )}
      </Stack>
    </SlideUp>
  );
}
