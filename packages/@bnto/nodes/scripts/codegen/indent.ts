/** Indent every non-blank line of `s` by `n` two-space steps. */
export function indent(s: string, n: number): string {
  const pad = "  ".repeat(n);
  return s
    .split("\n")
    .map((line) => (line.trim() ? pad + line : line))
    .join("\n");
}
