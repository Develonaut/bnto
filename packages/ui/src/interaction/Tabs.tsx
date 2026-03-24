"use client";

import { useState, useContext, createContext, useCallback, forwardRef } from "react";
import type { ElementRef, ComponentPropsWithoutRef, ComponentProps } from "react";

import * as TabsPrimitive from "@radix-ui/react-tabs";

import { Button } from "./Button";
import { cn } from "../utils/cn";

/* ── Context (surfaces active value to Triggers) ───────────── */

const TabsValueContext = createContext<string | undefined>(undefined);

/* ── Root ───────────────────────────────────────────────────── */

export function Tabs({
  value: controlledValue,
  defaultValue,
  onValueChange,
  ...props
}: ComponentProps<typeof TabsPrimitive.Root>) {
  const [internalValue, setInternalValue] = useState(defaultValue);
  const activeValue = controlledValue ?? internalValue;

  const handleValueChange = useCallback(
    (v: string) => {
      if (controlledValue === undefined) setInternalValue(v);
      onValueChange?.(v);
    },
    [controlledValue, onValueChange],
  );

  return (
    <TabsValueContext.Provider value={activeValue}>
      <TabsPrimitive.Root value={activeValue} onValueChange={handleValueChange} {...props} />
    </TabsValueContext.Provider>
  );
}

/* ── List ───────────────────────────────────────────────────── */

export const TabsList = forwardRef<
  ElementRef<typeof TabsPrimitive.List>,
  ComponentPropsWithoutRef<typeof TabsPrimitive.List>
>(({ className, ...props }, ref) => (
  <TabsPrimitive.List
    ref={ref}
    className={cn(
      "inline-flex items-center gap-1 rounded-full bg-input border border-border p-1",
      className,
    )}
    {...props}
  />
));
TabsList.displayName = "Tabs.List";

/* ── Trigger ────────────────────────────────────────────────── */

export const TabsTrigger = forwardRef<
  ElementRef<typeof TabsPrimitive.Trigger>,
  ComponentPropsWithoutRef<typeof TabsPrimitive.Trigger>
>(({ value, children, ...props }, ref) => {
  const activeValue = useContext(TabsValueContext);
  const isActive = activeValue === value;

  return (
    <span className={isActive ? "inline-flex" : "group inline-flex p-1 -m-0.5"}>
      <TabsPrimitive.Trigger ref={ref} value={value} asChild {...props}>
        <Button
          variant={isActive ? "outline" : "ghost"}
          data-dormant={!isActive ? "" : undefined}
          className="rounded-full"
        >
          {children}
        </Button>
      </TabsPrimitive.Trigger>
    </span>
  );
});
TabsTrigger.displayName = "Tabs.Trigger";

/* ── Content ────────────────────────────────────────────────── */

export const TabsContent = forwardRef<
  ElementRef<typeof TabsPrimitive.Content>,
  ComponentPropsWithoutRef<typeof TabsPrimitive.Content>
>(({ className, ...props }, ref) => (
  <TabsPrimitive.Content ref={ref} className={cn("mt-2 focus-ring", className)} {...props} />
));
TabsContent.displayName = "Tabs.Content";
