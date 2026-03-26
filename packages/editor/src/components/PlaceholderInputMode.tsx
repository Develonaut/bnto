/** Placeholder for unimplemented input modes (text, url). */
function PlaceholderInputMode({ label }: { label: string }) {
  return (
    <div className="rounded-lg border border-dashed border-border p-8 text-center text-muted-foreground">
      {label}
    </div>
  );
}

export { PlaceholderInputMode };
