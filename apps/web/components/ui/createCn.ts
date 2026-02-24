import { tv } from "tailwind-variants";

import { cn } from "#lib/utils";

/**
 * Wraps `tv()` from tailwind-variants so we can swap the underlying library
 * without touching consumers. Returns a function that resolves variant props
 * to a merged class string.
 *
 * Usage:
 *   const stackCn = createCn({ base: "flex", variants: { ... } });
 *   <div className={stackCn({ gap: "md" }, className)} />
 */
export function createCn(config: Parameters<typeof tv>[0]) {
  const variants = tv(config);
  type Props = Parameters<typeof variants>[0];
  return (props: Props, ...classNames: (string | undefined)[]) =>
    cn(variants(props), ...classNames);
}
