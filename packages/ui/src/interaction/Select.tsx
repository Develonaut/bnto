"use client";

import type { ComponentProps, ReactNode } from "react";

import * as SelectPrimitive from "@radix-ui/react-select";
import { CheckIcon, ChevronDownIcon, ChevronUpIcon } from "../icons";
import { ITEM_CN } from "../layout/List";
import { Popup } from "../overlay/Popup";
import { PopupTriggerButton } from "./PopupTriggerButton";
import { POPUP_OFFSET_PX, type PopupOffset } from "../overlay/popupOffset";
import type { SurfaceElevation } from "../surface/Surface";

import { cn } from "../utils/cn";

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

function SelectContentInner({
  children,
  position,
  elevation,
  className,
}: {
  children: ReactNode;
  position: string;
  elevation: SurfaceElevation;
  className?: string;
}) {
  return (
    <Popup
      originStyle="var(--radix-select-content-transform-origin)"
      elevation={elevation}
      className={className}
    >
      <div className="overflow-x-hidden overflow-y-auto">
        <SelectScrollUpButton />
        <SelectPrimitive.Viewport
          className={cn(
            "p-1",
            position === "popper" && "w-full min-w-[var(--radix-select-trigger-width)] scroll-my-1",
          )}
        >
          {children}
        </SelectPrimitive.Viewport>
        <SelectScrollDownButton />
      </div>
    </Popup>
  );
}

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

export function SelectScrollUpButton({
  className,
  ...props
}: ComponentProps<typeof SelectPrimitive.ScrollUpButton>) {
  return (
    <SelectPrimitive.ScrollUpButton
      data-slot="select-scroll-up-button"
      className={cn("flex cursor-default items-center justify-center py-1", className)}
      {...props}
    >
      <ChevronUpIcon className="size-4" />
    </SelectPrimitive.ScrollUpButton>
  );
}

export function SelectScrollDownButton({
  className,
  ...props
}: ComponentProps<typeof SelectPrimitive.ScrollDownButton>) {
  return (
    <SelectPrimitive.ScrollDownButton
      data-slot="select-scroll-down-button"
      className={cn("flex cursor-default items-center justify-center py-1", className)}
      {...props}
    >
      <ChevronDownIcon className="size-4" />
    </SelectPrimitive.ScrollDownButton>
  );
}

export function Select({ ...props }: ComponentProps<typeof SelectPrimitive.Root>) {
  return <SelectPrimitive.Root data-slot="select" {...props} />;
}
