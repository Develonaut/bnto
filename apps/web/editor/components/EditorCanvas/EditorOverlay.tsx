import type { HTMLAttributes } from "react";
import { cn } from "@bnto/ui";

/**
 * EditorOverlay — floating panel layer over the canvas.
 *
 * pointer-events-none container with consistent inset padding.
 * Children (panels, toolbar) position themselves within this layer.
 */

function EditorOverlay({
  className,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "pointer-events-none absolute inset-4 top-[6rem] z-10",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

export { EditorOverlay };
