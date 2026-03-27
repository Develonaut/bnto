/**
 * URL search param state for the explore page (?category=...).
 */

"use client";

import { useCallback, useTransition } from "react";
import { useRouter, useSearchParams } from "next/navigation";

export function useExploreParams() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [, startTransition] = useTransition();

  const update = useCallback(
    (key: string, value: string) => {
      const params = new URLSearchParams(searchParams.toString());
      if (!value || value === "all") params.delete(key);
      else params.set(key, value);
      const qs = params.toString();
      startTransition(() => router.replace(`/explore${qs ? `?${qs}` : ""}`, { scroll: false }));
    },
    [router, searchParams, startTransition],
  );

  return {
    category: searchParams.get("category") ?? "all",
    update,
  };
}
