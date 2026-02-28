import type { ComponentProps } from "react";

import { cn } from "@/lib/cn";

import { Container } from "./Container";
import { createCn } from "./createCn";

/* ── AppShell ──────────────────────────────────────────────────
 * Application shell with composable layout parts.
 *
 *   // Root layout:
 *   <AppShell>
 *     <AppShell.Header><Navbar /></AppShell.Header>
 *     {children}
 *   </AppShell>
 *
 *   // Route group layout:
 *   <AppShell.Main>
 *     {children}
 *   </AppShell.Main>
 *
 *   // Page:
 *   <AppShell.Content>
 *     <Hero />
 *     <FeatureSection />
 *   </AppShell.Content>
 * ────────────────────────────────────────────────────────────── */

/* ── Root ──────────────────────────────────────────────────── */

function AppShellRoot({ className, ...props }: ComponentProps<"div">) {
  return (
    <div className={cn("flex min-h-svh flex-col", className)} {...props} />
  );
}

/* ── Header ───────────────────────────────────────────────── */

function AppShellHeader({ className, ...props }: ComponentProps<"header">) {
  return <header className={cn("relative z-50", className)} {...props} />;
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

function AppShellMain({ clearance, className, ...props }: AppShellMainProps) {
  return <main className={mainCn({ clearance }, className)} {...props} />;
}

/* ── Content ──────────────────────────────────────────────── */

function AppShellContent({
  className,
  children,
  ...props
}: ComponentProps<"div">) {
  return (
    <div className={cn("flex-1 py-12", className)} {...props}>
      <Container>
        <div className="flex flex-col gap-20">{children}</div>
      </Container>
    </div>
  );
}

/* ── Export ────────────────────────────────────────────────── */

export const AppShell = Object.assign(AppShellRoot, {
  Root: AppShellRoot,
  Header: AppShellHeader,
  Main: AppShellMain,
  Content: AppShellContent,
});
