import {
  Button as PrimitiveButton,
  type ButtonProps,
} from "../primitives/button";
import { type ComponentProps, forwardRef } from "react";

const Button = forwardRef<
  HTMLButtonElement,
  ComponentProps<typeof PrimitiveButton>
>((props, ref) => <PrimitiveButton ref={ref} {...props} />);
Button.displayName = "Button";

export { Button };
export type { ButtonProps };
