export function SizeDisplay({
  originalSize,
  savings,
  outputSize,
}: {
  originalSize: string | undefined;
  savings: string | undefined;
  outputSize: string;
}) {
  if (originalSize != null && savings != null) {
    return (
      <>
        <span className="line-through" data-testid="original-size">
          {originalSize}
        </span>{" "}
        <span className="font-semibold text-primary">{savings}</span> {outputSize}
      </>
    );
  }
  return <>{outputSize}</>;
}
