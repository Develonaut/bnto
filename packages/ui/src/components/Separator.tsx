import { Separator as PrimitiveSeparator } from "../primitives/separator";
import { type ComponentProps, forwardRef } from "react";

const Separator = forwardRef<
  HTMLDivElement,
  ComponentProps<typeof PrimitiveSeparator>
>((props, ref) => <PrimitiveSeparator ref={ref} {...props} />);
Separator.displayName = "Separator";

export { Separator };
