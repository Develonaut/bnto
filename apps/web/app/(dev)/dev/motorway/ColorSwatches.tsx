import { cn } from "@/lib/cn";
import { Card } from "@/components/ui/Card";

const swatches = [
  { name: "Primary", bg: "bg-primary", fg: "text-primary-foreground", depth: "depth-primary" },
  { name: "Secondary", bg: "bg-secondary", fg: "text-secondary-foreground", depth: "depth-secondary" },
  { name: "Accent", bg: "bg-accent", fg: "text-accent-foreground", depth: "depth-accent" },
  { name: "Muted", bg: "bg-muted", fg: "text-muted-foreground", depth: "depth-muted" },
  { name: "Destructive", bg: "bg-destructive", fg: "text-destructive-foreground", depth: "depth-destructive" },
  { name: "Success", bg: "bg-success", fg: "text-success-foreground", depth: "depth-success" },
  { name: "Warning", bg: "bg-warning", fg: "text-warning-foreground", depth: "depth-warning" },
  { name: "Card", bg: "bg-card", fg: "text-card-foreground", depth: "depth-outline" },
];

export function ColorSwatches() {
  return (
    <div className="grid grid-cols-4 gap-4">
      {swatches.map(({ name, bg, fg, depth }) => (
        <Card
          key={name}
          className={cn(
            "flex h-20 items-center justify-center rounded-xl",
            bg,
            fg,
            depth,
          )}
        >
          <span className="text-sm font-semibold">{name}</span>
        </Card>
      ))}
    </div>
  );
}
