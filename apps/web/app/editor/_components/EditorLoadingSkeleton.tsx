import { Skeleton } from "@bnto/ui";

/**
 * Editor loading skeleton — mirrors the canvas layout.
 *
 * Shows the grid background + a ghost toolbar pill at the bottom
 * so the loading state feels like the editor is about to appear.
 */
export function EditorLoadingSkeleton() {
  return (
    <div className="relative h-full overflow-hidden bg-background">
      {/* Grid background — matches ReactFlow <Background gap={40} color={--border} /> */}
      <div
        className="absolute inset-0"
        style={{
          backgroundImage:
            "linear-gradient(var(--border) 1px, transparent 1px), linear-gradient(90deg, var(--border) 1px, transparent 1px)",
          backgroundSize: "40px 40px",
        }}
      />

      {/* Ghost toolbar pill — bottom-center, same position as EditorToolbar */}
      <div className="absolute inset-4 top-[6rem]">
        <div className="absolute bottom-0 left-1/2 -translate-x-1/2">
          <Skeleton className="h-12 w-80 rounded-full" />
        </div>
      </div>
    </div>
  );
}
