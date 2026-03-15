"use client";

import { useCallback, useState } from "react";

import { CopyIcon, CheckIcon } from "../icons";
import { Button } from "./Button";

type CopyButtonProps = {
  /** The text to copy to the clipboard. */
  value: string;
  /** Label for screen readers. */
  label?: string;
};

/**
 * CopyButton — outline icon button that copies text to the clipboard.
 *
 * Swaps to a check icon for 1.5s after copy.
 */
function CopyButton({ value, label = "Copy to clipboard" }: CopyButtonProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = useCallback(() => {
    navigator.clipboard.writeText(value).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    });
  }, [value]);

  return (
    <Button variant="outline" size="icon" onClick={handleCopy} aria-label={label}>
      {copied ? <CheckIcon /> : <CopyIcon />}
    </Button>
  );
}

export { CopyButton };
