"use client";

import type { ComponentProps } from "react";
import { useState, useCallback } from "react";

import { cn } from "../../utils/cn";
import { Input } from "../Input";
import { Row } from "../../layout/Row";
import { PasswordToggle } from "./PasswordToggle";

function PasswordInput({
  className,
  wrapperClassName,
  disabled,
  ...props
}: Omit<ComponentProps<"input">, "type"> & { wrapperClassName?: string }) {
  const [visible, setVisible] = useState(false);
  const toggleVisible = useCallback(() => setVisible((v) => !v), []);

  return (
    <Row className="gap-2">
      <Input
        type={visible ? "text" : "password"}
        className={className}
        wrapperClassName={cn("min-w-0 flex-1", wrapperClassName)}
        disabled={disabled}
        {...props}
      />
      <PasswordToggle visible={visible} disabled={disabled} onToggle={toggleVisible} />
    </Row>
  );
}

export { PasswordInput };
