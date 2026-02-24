import { Button } from "#components/ui/button";

const items = [
  { title: "Compress Images", variant: "default" as const },
  { title: "Clean CSV", variant: "secondary" as const },
  { title: "Rename Files", variant: "muted" as const },
  { title: "Convert Format", variant: "default" as const },
  { title: "Call API", variant: "secondary" as const },
  { title: "Resize Images", variant: "muted" as const },
];

export function ToolGrid() {
  return (
    <div className="grid grid-cols-3 gap-4">
      {items.map((item) => (
        <Button
          key={item.title}
          variant={item.variant}
          className="h-28 w-full rounded-xl font-display font-semibold"
        >
          {item.title}
        </Button>
      ))}
    </div>
  );
}
