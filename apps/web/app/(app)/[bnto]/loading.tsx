import { Container, Skeleton } from "@bnto/ui";

/**
 * Streaming boundary for recipe pages.
 *
 * Mirrors the new hero layout: mascot, H1, dropzone above fold.
 * No stepper skeleton — stepper is hidden until files are added.
 */
export default function BntoLoading() {
  return (
    <Container size="sm" className="space-y-6 text-center py-20">
      {/* mascot placeholder */}
      <div className="flex justify-center">
        <Skeleton className="size-28 rounded-full sm:size-36" />
      </div>

      {/* h1 */}
      <Skeleton className="mx-auto h-9 w-72" />

      {/* dropzone */}
      <Skeleton className="mx-auto h-52 w-full rounded-xl" />
    </Container>
  );
}
