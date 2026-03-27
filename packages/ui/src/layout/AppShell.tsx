import type { ComponentProps } from "react";

import { cn } from "../utils/cn";

import { Container } from "./Container";
import { createCn } from "../utils/createCn";

/* ── AppShell ──────────────────────────────────────────────────
 * Application shell with composable layout parts.
 *
 *   // Root layout:
 *   <AppShell>
 *     <AppShellHeader><Navbar /></AppShellHeader>
 *     {children}
 *   </AppShell>
 *
 *   // Route group layout:
 *   <AppShellMain>{children}</AppShellMain>
 *
 *   // Page:
 *   <AppShellContent>
 *     <Hero />
 *     <FeatureSection />
 *   </AppShellContent>
 * ────────────────────────────────────────────────────────────── */

/* ── Root ──────────────────────────────────────────────────── */

export function AppShell({ className, ...props }: ComponentProps<"div">) {
  return <div className={cn("flex min-h-svh flex-col", className)} {...props} />;
}

/* ── Header ───────────────────────────────────────────────── */

export function AppShellHeader({ className, ...props }: ComponentProps<"header">) {
  return <header className={cn("relative z-sticky", className)} {...props} />;
}

/* ── Main ─────────────────────────────────────────────────── */

const mainCn = createCn({
  base: "flex flex-1 flex-col",
  variants: {
    clearance: {
      default: "pt-20",
      none: "",
    },
  },
  defaultVariants: {
    clearance: "default",
  },
});

type AppShellMainProps = ComponentProps<"main"> & {
  /** Top padding to clear the fixed navbar. Default `"default"`. */
  clearance?: "default" | "none";
};

export function AppShellMain({ clearance, className, ...props }: AppShellMainProps) {
  return <main className={mainCn({ clearance }, className)} {...props} />;
}

/* ── Content ──────────────────────────────────────────────── */

const GAP_MAP = {
  sm: "gap-8",
  md: "gap-20",
} as const;

type AppShellContentProps = ComponentProps<"div"> & {
  /** Vertical gap between content sections. Default `"md"`. */
  gap?: keyof typeof GAP_MAP;
};

export function AppShellContent({
  gap = "md",
  className,
  children,
  ...props
}: AppShellContentProps) {
  return (
    <div className={cn("min-h-[80svh] flex-1 py-12", className)} {...props}>
      <Container>
        <div className={cn("flex flex-col", GAP_MAP[gap])}>{children}</div>
      </Container>
    </div>
  );
}
