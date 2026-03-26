"use client";

import { useCallback } from "react";

export function useGateHandlers(setOpen: (open: boolean) => void) {
  const handleGateClick = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      e.stopPropagation();
      setOpen(true);
    },
    [setOpen],
  );

  const handleGateKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        e.stopPropagation();
        setOpen(true);
      }
    },
    [setOpen],
  );

  return { handleGateClick, handleGateKeyDown };
}
