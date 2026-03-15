import { Suspense } from "react";
import { Skeleton, Stack, Text } from "@bnto/ui";

import { EditorShell } from "./_components/EditorShell";

/**
 * /editor — full-viewport recipe editor.
 *
 * Server component page. The layout (navbar, viewport chrome) renders on
 * the server. EditorShell is a client island that owns searchParams,
 * editor state, and the ReactFlow canvas.
 *
 * Wrapped in Suspense because EditorShell uses useSearchParams() which
 * requires a Suspense boundary above it in App Router.
 */
export default function EditorPage() {
  return (
    <Suspense
      fallback={
        <Stack className="h-full items-center justify-center gap-4">
          <Skeleton className="h-8 w-48" />
          <Text color="muted" size="sm">
            Loading editor…
          </Text>
        </Stack>
      }
    >
      <EditorShell />
    </Suspense>
  );
}
