"use client";

import type { MouseEvent } from "react";

import { Button } from "../Button";
import { EyeIcon, EyeOffIcon } from "../../icons";

/** Prevent mousedown from stealing focus -- keeps focus on the text input. */
function preventFocus(e: MouseEvent) {
  e.preventDefault();
}

interface PasswordToggleProps {
  visible: boolean;
  disabled?: boolean;
  onToggle: () => void;
}

export function PasswordToggle({ visible, disabled, onToggle }: PasswordToggleProps) {
  return (
    <Button
      type="button"
      variant="outline"
      size="icon"
      disabled={disabled}
      tabIndex={-1}
      onMouseDown={preventFocus}
      onClick={onToggle}
      aria-label={visible ? "Hide password" : "Show password"}
    >
      {visible ? <EyeOffIcon /> : <EyeIcon />}
    </Button>
  );
}
