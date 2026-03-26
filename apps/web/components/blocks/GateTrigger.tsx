import type { ReactNode } from "react";

interface GateTriggerProps {
  onClickCapture: (e: React.MouseEvent) => void;
  onKeyDownCapture: (e: React.KeyboardEvent) => void;
  children: ReactNode;
}

export function GateTrigger({ onClickCapture, onKeyDownCapture, children }: GateTriggerProps) {
  return (
    <span
      role="button"
      tabIndex={0}
      onClickCapture={onClickCapture}
      onKeyDownCapture={onKeyDownCapture}
      className="inline-flex cursor-pointer"
    >
      {children}
    </span>
  );
}
