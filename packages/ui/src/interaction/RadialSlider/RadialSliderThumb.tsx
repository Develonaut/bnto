import type { Ref } from "react";

import { GripVerticalIcon } from "../../icons";
import { Button } from "../Button";

interface RadialSliderThumbProps {
  isDragging: boolean;
  isHovering: boolean;
  thumbRef: Ref<HTMLDivElement>;
}

/** Default grip-icon thumb for the radial slider. */
export function RadialSliderThumb({ isDragging, isHovering, thumbRef }: RadialSliderThumbProps) {
  return (
    <Button
      asChild
      variant="primary"
      elevation="sm"
      spring="bouncy"
      pressed={isDragging}
      hovered={isHovering}
      className="rounded-full"
    >
      <div ref={thumbRef} className="flex items-center justify-center size-8 ring-0">
        <GripVerticalIcon strokeWidth={3} className="size-3.5 shrink-0" />
      </div>
    </Button>
  );
}
