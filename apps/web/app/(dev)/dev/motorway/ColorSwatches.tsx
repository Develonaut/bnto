import { cn } from "@/lib/cn";
import { Card } from "@/components/ui/Card";

const swatches = [
  { name: "Primary", bg: "bg-primary", fg: "text-primary-foreground", surface: "surface-primary" },
  { name: "Secondary", bg: "bg-secondary", fg: "text-secondary-foreground", surface: "surface-secondary" },
  { name: "Accent", bg: "bg-accent", fg: "text-accent-foreground", surface: "surface-accent" },
  { name: "Muted", bg: "bg-muted", fg: "text-muted-foreground", surface: "surface-muted" },
  { name: "Destructive", bg: "bg-destructive", fg: "text-destructive-foreground", surface: "surface-destructive" },
  { name: "Success", bg: "bg-success", fg: "text-success-foreground", surface: "surface-success" },
  { name: "Warning", bg: "bg-warning", fg: "text-warning-foreground", surface: "surface-warning" },
  { name: "Card", bg: "bg-card", fg: "text-card-foreground", surface: "surface-outline" },
];

export function ColorSwatches() {
  return (
    <div className="grid grid-cols-4 gap-4">
      {swatches.map(({ name, bg, fg, surface }) => (
        <Card
          key={name}
          className={cn(
            "flex h-20 items-center justify-center rounded-xl",
            bg,
            fg,
            surface,
          )}
        >
          <span className="text-sm font-semibold">{name}</span>
        </Card>
      ))}
    </div>
  );
}
