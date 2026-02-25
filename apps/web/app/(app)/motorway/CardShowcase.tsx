import { Card } from "@/components/ui/Card";

const TIERS: { label: string; depth: "none" | "sm" | "md" | "lg" }[] = [
  { label: "none", depth: "none" },
  { label: "sm", depth: "sm" },
  { label: "md (default)", depth: "md" },
  { label: "lg", depth: "lg" },
];

export function CardShowcase() {
  return (
    <div className="grid grid-cols-4 gap-6">
      {TIERS.map(({ label, depth }) => (
        <Card
          key={label}
          depth={depth}
          className="flex h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid={`depth-card-${depth}`}
        >
          {label}
        </Card>
      ))}
    </div>
  );
}
