/**
 * Navigation button with built-in optimistic pressed state.
 *
 * Uses the ghost variant (border disappears when pressed) and
 * tracks navigation to show an instant pressed effect on click.
 * All Button props except `variant` and `pressed` are forwarded.
 */

"use client";

import { useEffect, useState } from "react";

import { usePathname } from "next/navigation";

import { Button } from "@bnto/ui";

export function NavButton({
  href,
  onClick,
  ...props
}: { href: string } & Omit<
  React.ComponentProps<typeof Button>,
  "variant" | "pressed" | "href"
>) {
  const pathname = usePathname();
  const [pendingPath, setPendingPath] = useState<string | null>(null);

  // eslint-disable-next-line react-hooks/set-state-in-effect -- clearing optimistic state on navigation complete
  useEffect(() => setPendingPath(null), [pathname]);

  return (
    <Button
      variant="ghost"
      elevation="sm"
      {...props}
      href={href}
      pressed={pathname === href || pendingPath === href}
      onClick={(e: React.MouseEvent) => {
        setPendingPath(href);
        (onClick as ((e: React.MouseEvent) => void) | undefined)?.(e);
      }}
    />
  );
}
