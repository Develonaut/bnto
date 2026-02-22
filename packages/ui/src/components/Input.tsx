import { Input as PrimitiveInput } from "../primitives/input";
import { type ComponentProps, forwardRef } from "react";

const Input = forwardRef<
  HTMLInputElement,
  ComponentProps<typeof PrimitiveInput>
>((props, ref) => <PrimitiveInput ref={ref} {...props} />);
Input.displayName = "Input";

export { Input };
