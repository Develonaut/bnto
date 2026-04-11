import { Container, Skeleton } from "@bnto/ui";

/**
 * Streaming boundary for recipe pages.
 *
 * Mirrors the new hero layout: H1 + dropzone with mascot overlay.
 * No stepper skeleton — stepper is hidden until files are added.
 */
export default function BntoLoading() {
  return (
    <Container size="sm" className="space-y-6 text-center py-20">
      {/* h1 */}
      <Skeleton className="mx-auto h-9 w-72" />

      {/* dropzone (mascot overlays it, not skeletonized) */}
      <Skeleton className="mx-auto h-52 w-full rounded-xl" />
    </Container>
  );
}
