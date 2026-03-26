"use client";

import type { ComponentProps } from "react";

import * as SelectPrimitive from "@radix-ui/react-select";
import { CheckIcon, ChevronDownIcon } from "../../icons";
import { ITEM_CN } from "../../layout/List";
import { PopupTriggerButton } from "../PopupTriggerButton";
import { POPUP_OFFSET_PX, type PopupOffset } from "../../overlay/popupOffset";
import type { SurfaceElevation } from "../../surface/Surface";
import { SelectContentInner } from "./SelectContentInner";

import { cn } from "../../utils/cn";

export { SelectScrollUpButton, SelectScrollDownButton } from "./SelectScrollButtons";

export function SelectGroup({ ...props }: ComponentProps<typeof SelectPrimitive.Group>) {
  return <SelectPrimitive.Group data-slot="select-group" {...props} />;
}

export function SelectValue({ ...props }: ComponentProps<typeof SelectPrimitive.Value>) {
  return <SelectPrimitive.Value data-slot="select-value" {...props} />;
}

export function SelectTrigger({
  className,
  children,
  ...props
}: ComponentProps<typeof SelectPrimitive.Trigger>) {
  return (
    <SelectPrimitive.Trigger asChild data-slot="select-trigger" {...props}>
      <PopupTriggerButton
        className={cn(
          "w-fit justify-between font-normal data-[placeholder]:text-muted-foreground *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2",
          className,
        )}
      >
        {children}
        <SelectPrimitive.Icon asChild>
          <ChevronDownIcon className="size-4 shrink-0 opacity-50" />
        </SelectPrimitive.Icon>
      </PopupTriggerButton>
    </SelectPrimitive.Trigger>
  );
}

type SelectContentProps = ComponentProps<typeof SelectPrimitive.Content> & {
  /** Offset from trigger edge. Default "md" (16px). */
  offset?: PopupOffset;
  /** Card elevation. Default "lg". */
  elevation?: SurfaceElevation;
};

export function SelectContent({
  className,
  children,
  position = "popper",
  align = "center",
  offset = "md",
  elevation = "lg",
  ...props
}: SelectContentProps) {
  return (
    <SelectPrimitive.Portal>
      <SelectPrimitive.Content
        data-slot="select-content"
        className="z-dropdown max-h-(--radix-select-content-available-height) min-w-[8rem] origin-(--radix-select-content-transform-origin) outline-hidden"
        position={position}
        align={align}
        sideOffset={POPUP_OFFSET_PX[offset]}
        {...props}
      >
        <SelectContentInner position={position} elevation={elevation} className={className}>
          {children}
        </SelectContentInner>
      </SelectPrimitive.Content>
    </SelectPrimitive.Portal>
  );
}

export function SelectLabel({ className, ...props }: ComponentProps<typeof SelectPrimitive.Label>) {
  return (
    <SelectPrimitive.Label
      data-slot="select-label"
      className={cn("text-muted-foreground px-2 py-1.5 text-xs", className)}
      {...props}
    />
  );
}

export function SelectItem({
  className,
  children,
  ...props
}: ComponentProps<typeof SelectPrimitive.Item>) {
  return (
    <SelectPrimitive.Item
      data-slot="select-item"
      className={cn(
        ITEM_CN,
        "cursor-default rounded-lg hover:bg-muted focus-visible:bg-muted data-[disabled]:pointer-events-none data-[disabled]:opacity-50 [&_svg]:pointer-events-none [&_svg]:shrink-0",
        className,
      )}
      {...props}
    >
      <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
      <span className="ml-auto flex size-3.5 items-center justify-center">
        <SelectPrimitive.ItemIndicator>
          <CheckIcon className="size-4 text-muted-foreground" />
        </SelectPrimitive.ItemIndicator>
      </span>
    </SelectPrimitive.Item>
  );
}

export function SelectSeparator({
  className,
  ...props
}: ComponentProps<typeof SelectPrimitive.Separator>) {
  return (
    <SelectPrimitive.Separator
      data-slot="select-separator"
      className={cn("bg-border pointer-events-none -mx-1 my-1 h-px", className)}
      {...props}
    />
  );
}

export function Select({ ...props }: ComponentProps<typeof SelectPrimitive.Root>) {
  return <SelectPrimitive.Root data-slot="select" {...props} />;
}
