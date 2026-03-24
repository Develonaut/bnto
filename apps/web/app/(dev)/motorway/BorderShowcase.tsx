import { PlusIcon, Surface, Text } from "@bnto/ui";

export function BorderShowcase() {
  return (
    <div className="grid grid-cols-3 gap-4">
      <Surface
        elevation="none"
        border="solid"
        rounded="xl"
        className="flex min-h-24 flex-col items-center justify-center gap-1.5"
      >
        <Text size="sm" className="font-display font-medium">
          solid (default)
        </Text>
      </Surface>

      <Surface
        elevation="none"
        border="dashed"
        rounded="xl"
        className="flex min-h-24 flex-col items-center justify-center gap-1.5"
      >
        <PlusIcon className="size-6 text-muted-foreground/40" />
        <Text size="sm" color="muted" className="font-display font-medium">
          dashed
        </Text>
      </Surface>

      <Surface
        elevation="none"
        border="none"
        rounded="xl"
        className="flex min-h-24 flex-col items-center justify-center gap-1.5 bg-muted"
      >
        <Text size="sm" color="muted" className="font-display font-medium">
          none
        </Text>
      </Surface>
    </div>
  );
}
