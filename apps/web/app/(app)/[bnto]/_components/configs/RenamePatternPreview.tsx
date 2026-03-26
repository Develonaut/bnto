/** Renders a live preview of the rename pattern with sample substitutions. */
export function RenamePatternPreview({ pattern }: { pattern: string }) {
  if (!pattern) return null;

  return (
    <span className="text-muted-foreground shrink-0 text-xs">
      <span className="font-mono">
        {pattern.replace("{{name}}", "photo").replace("{{ext}}", ".png")}
      </span>
    </span>
  );
}
