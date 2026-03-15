"use client";

import { ErrorReport } from "@/components/blocks/ErrorReport";

export default function DevError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  return <ErrorReport error={error} reset={reset} />;
}
