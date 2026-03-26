"use client";

import type { ComponentProps, MouseEvent } from "react";
import { useState, useCallback } from "react";

import { cn } from "../utils/cn";
import { Button } from "./Button";
import { Input } from "./Input";
import { Row } from "../layout/Row";
import { EyeIcon, EyeOffIcon } from "../icons";

/** Prevent mousedown from stealing focus -- keeps focus on the text input. */
function preventFocus(e: MouseEvent) {
  e.preventDefault();
}

function PasswordInput({
  className,
  wrapperClassName,
  disabled,
  ...props
}: Omit<ComponentProps<"input">, "type"> & { wrapperClassName?: string }) {
  const [visible, setVisible] = useState(false);
  const toggleVisible = useCallback(() => setVisible((v) => !v), []);
  const icon = visible ? <EyeOffIcon /> : <EyeIcon />;

  return (
    <Row className="gap-2">
      <Input
        type={visible ? "text" : "password"}
        className={className}
        wrapperClassName={cn("min-w-0 flex-1", wrapperClassName)}
        disabled={disabled}
        {...props}
      />
      <Button
        type="button"
        variant="outline"
        size="icon"
        disabled={disabled}
        tabIndex={-1}
        onMouseDown={preventFocus}
        onClick={toggleVisible}
        aria-label={visible ? "Hide password" : "Show password"}
      >
        {icon}
      </Button>
    </Row>
  );
}

export { PasswordInput };
