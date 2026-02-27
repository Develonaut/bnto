import { Container } from "@/components/ui/Container";
import { Skeleton } from "@/components/ui/Skeleton";

/**
 * Streaming boundary for recipe pages.
 *
 * Mirrors the RecipeShell layout so content "paints in" without
 * layout shift when the page chunk finishes loading.
 */
export default function BntoLoading() {
  return (
    <Container size="sm" className="space-y-6 text-center py-20">
      {/* h1 */}
      <Skeleton className="mx-auto h-9 w-72" />

      {/* description */}
      <div className="mx-auto flex max-w-xl flex-col items-center gap-2">
        <Skeleton className="h-5 w-96 max-w-full" />
        <Skeleton className="h-5 w-64" />
      </div>

      {/* phase indicator (3 circles + lines) */}
      <div className="flex items-center justify-center gap-2">
        <Skeleton className="size-7 rounded-full" />
        <Skeleton className="h-0.5 w-10" />
        <Skeleton className="size-7 rounded-full" />
        <Skeleton className="h-0.5 w-10" />
        <Skeleton className="size-7 rounded-full" />
      </div>

      {/* dropzone */}
      <Skeleton className="mx-auto h-40 w-full rounded-xl" />
    </Container>
  );
}
