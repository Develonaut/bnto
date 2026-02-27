import { Card } from "@/components/ui/Card";

const TIERS: { label: string; elevation: "none" | "sm" | "md" | "lg" }[] = [
  { label: "none", elevation: "none" },
  { label: "sm", elevation: "sm" },
  { label: "md (default)", elevation: "md" },
  { label: "lg", elevation: "lg" },
];

export function CardShowcase() {
  return (
    <div className="grid grid-cols-4 gap-6">
      {TIERS.map(({ label, elevation }) => (
        <Card
          key={label}
          elevation={elevation}
          className="flex h-24 items-center justify-center rounded-xl font-display font-semibold"
          data-testid={`elevation-card-${elevation}`}
        >
          {label}
        </Card>
      ))}
    </div>
  );
}
