import React from "react";

import { cn } from "@/lib/utils";

type BackgroundProps = {
  children: React.ReactNode;
  variant?: "top" | "bottom";
  className?: string;
};

export const Background = ({
  children,
  variant = "top",
  className,
}: BackgroundProps) => {
  return (
    <div
      className={cn(
        "relative mx-2.5 mt-2.5 lg:mx-4",
        variant === "top" && "rounded-t-4xl rounded-b-2xl",
        variant === "bottom" && "rounded-t-2xl rounded-b-4xl",
        className,
      )}
    >
      {children}
    </div>
  );
};
