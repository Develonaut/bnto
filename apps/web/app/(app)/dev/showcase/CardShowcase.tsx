import { Card } from "@/components/ui/Card";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

const sizes: { label: string; depth: "none" | "sm" | "md" | "lg" }[] = [
  { label: "depth-none", depth: "none" },
  { label: "depth-sm", depth: "sm" },
  { label: "depth-md (default)", depth: "md" },
  { label: "depth-lg", depth: "lg" },
];

export function CardShowcase() {
  return (
    <Stack gap="lg">
      {/* Row 1: Static depth cards (no pressable interaction) */}
      <div>
        <Text size="sm" weight="medium" color="muted" className="mb-3">
          Static depth &mdash; walls + ground shadow, no interaction
        </Text>
        <div className="grid grid-cols-4 gap-6">
          {sizes.map(({ label, depth }) => (
            <Card
              key={label}
              depth={depth}
              className="flex h-24 items-center justify-center rounded-xl font-display font-semibold"
              data-testid={`static-card-${label}`}
            >
              {label}
            </Card>
          ))}
        </div>
      </div>

      {/* Row 2: Pressable cards (hover to sink, active to flush) */}
      <div>
        <Text size="sm" weight="medium" color="muted" className="mb-3">
          Pressable &mdash; hover to sink, active pushes flush
        </Text>
        <div className="grid grid-cols-4 gap-6">
          {sizes.map(({ label, depth }) => (
            <Card
              key={label}
              depth={depth}
              className="flex h-24 cursor-pointer items-center justify-center rounded-xl font-display font-semibold pressable"
              data-testid={`pressable-card-${label}`}
            >
              {label}
            </Card>
          ))}
        </div>
      </div>
    </Stack>
  );
}
