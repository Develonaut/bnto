"use client";

import type { ComponentProps } from "react";
import { useState } from "react";

import { cn } from "../utils/cn";
import { Button } from "./Button";
import { Input } from "./Input";
import { Row } from "../layout/Row";
import { EyeIcon, EyeOffIcon } from "../icons";

/**
 * Password input with a toggle button beside it to reveal/hide the password.
 *
 * Renders an Input paired with an icon-sized toggle Button in a row.
 */
function PasswordInput({
  className,
  wrapperClassName,
  disabled,
  ...props
}: Omit<ComponentProps<"input">, "type"> & { wrapperClassName?: string }) {
  const [visible, setVisible] = useState(false);

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
        elevation="md"
        toggle
        pressed={visible}
        disabled={disabled}
        tabIndex={-1}
        onClick={() => setVisible((v) => !v)}
        aria-label={visible ? "Hide password" : "Show password"}
        aria-pressed={visible}
      >
        {visible ? (
          <EyeIcon className="size-4" />
        ) : (
          <EyeOffIcon className="size-4" />
        )}
      </Button>
    </Row>
  );
}

export { PasswordInput };
