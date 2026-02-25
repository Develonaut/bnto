"use client";

import * as React from "react";
import { useState, useContext, createContext, useCallback } from "react";

import * as TabsPrimitive from "@radix-ui/react-tabs";

import { buttonVariants } from "./button";
import { cn } from "@/lib/utils";

/* ── Context (surfaces active value to Triggers) ───────────── */

const TabsValueContext = createContext<string | undefined>(undefined);

/* ── Root ───────────────────────────────────────────────────── */

function TabsRoot({
  value: controlledValue,
  defaultValue,
  onValueChange,
  ...props
}: React.ComponentProps<typeof TabsPrimitive.Root>) {
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
      <TabsPrimitive.Root
        value={activeValue}
        onValueChange={handleValueChange}
        {...props}
      />
    </TabsValueContext.Provider>
  );
}

/* ── List ───────────────────────────────────────────────────── */

const TabsList = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.List>,
  React.ComponentPropsWithoutRef<typeof TabsPrimitive.List>
>(({ className, ...props }, ref) => (
  <TabsPrimitive.List
    ref={ref}
    className={cn(
      "inline-flex items-center gap-1 rounded-full border border-border/50 pt-1.5 pb-3 pl-3 pr-4",
      className,
    )}
    {...props}
  />
));
TabsList.displayName = "Tabs.List";

/* ── Trigger ────────────────────────────────────────────────── */

const TabsTrigger = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Trigger>,
  React.ComponentPropsWithoutRef<typeof TabsPrimitive.Trigger>
>(({ className, value, ...props }, ref) => {
  const activeValue = useContext(TabsValueContext);
  const isActive = activeValue === value;

  return (
    <TabsPrimitive.Trigger
      ref={ref}
      value={value}
      data-muted={!isActive || undefined}
      className={cn(
        "pressable",
        buttonVariants({ variant: "outline", size: "sm" }),
        className,
      )}
      {...props}
    />
  );
});
TabsTrigger.displayName = "Tabs.Trigger";

/* ── Content ────────────────────────────────────────────────── */

const TabsContent = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof TabsPrimitive.Content>
>(({ className, ...props }, ref) => (
  <TabsPrimitive.Content
    ref={ref}
    className={cn(
      "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
      className,
    )}
    {...props}
  />
));
TabsContent.displayName = "Tabs.Content";

/* ── Namespace ──────────────────────────────────────────────── */

export const Tabs = Object.assign(TabsRoot, {
  Root: TabsRoot,
  List: TabsList,
  Trigger: TabsTrigger,
  Content: TabsContent,
});

export { TabsList, TabsTrigger, TabsContent };
