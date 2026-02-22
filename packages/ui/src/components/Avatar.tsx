import {
  Avatar as PrimitiveAvatar,
  AvatarImage,
  AvatarFallback,
} from "../primitives/avatar";
import { type ComponentProps, forwardRef } from "react";

function AvatarRoot(props: ComponentProps<typeof PrimitiveAvatar>) {
  return <PrimitiveAvatar {...props} />;
}

const AvatarWithRef = forwardRef<
  HTMLSpanElement,
  ComponentProps<typeof PrimitiveAvatar>
>((props, ref) => <PrimitiveAvatar ref={ref} {...props} />);
AvatarWithRef.displayName = "Avatar";

export const Avatar = Object.assign(AvatarWithRef, {
  Root: AvatarRoot,
  Image: AvatarImage,
  Fallback: AvatarFallback,
});
