import type { RefObject, ReactNode } from "react";

import { RadialSliderThumb } from "./RadialSliderThumb";

interface ResolveThumbOptions {
  isDragging: boolean;
  isHovering: boolean;
  thumbRef: RefObject<HTMLDivElement | null>;
  renderThumb?: (props: { isDragging: boolean }) => ReactNode;
}

/** Pick custom or default thumb based on renderThumb prop. */
export function resolveThumb({
  isDragging,
  isHovering,
  thumbRef,
  renderThumb,
}: ResolveThumbOptions) {
  if (renderThumb) return renderThumb({ isDragging });
  return <RadialSliderThumb isDragging={isDragging} isHovering={isHovering} thumbRef={thumbRef} />;
}
