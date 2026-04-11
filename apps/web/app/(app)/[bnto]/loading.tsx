import { Container, Skeleton } from "@bnto/ui";

/**
 * Streaming boundary for recipe pages.
 *
 * Mirrors the Canva-style hero dropzone — all content inside one card.
 * No stepper skeleton — stepper is hidden until files are added.
 */
export default function BntoLoading() {
  return (
    <Container size="md" className="py-20 text-center">
      {/* Hero dropzone card with integrated content */}
      <Skeleton className="mx-auto h-80 w-full rounded-xl" />
    </Container>
  );
}
