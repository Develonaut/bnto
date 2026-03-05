"use client";

import {
  Button,
  Card,
  CheckCircle2Icon,
  DownloadIcon,
  FileIcon,
  ImageIcon,
  Row,
  Text,
} from "@bnto/ui";

/** Simulated file card for the mockups. */
export function MockFileCard({ name, size }: { name: string; size: string }) {
  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
    >
      <div className="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted">
        <ImageIcon className="size-4 text-muted-foreground" />
      </div>
      <div className="min-w-0 flex-1 text-left">
        <p className="truncate text-sm font-medium">{name}</p>
        <p className="text-xs text-muted-foreground">{size}</p>
      </div>
    </Card>
  );
}

/** Simulated result file card with download button. */
export function MockResultCard({
  name,
  original,
  compressed,
  savings,
}: {
  name: string;
  original: string;
  compressed: string;
  savings: string;
}) {
  return (
    <Card
      className="flex items-center gap-3 rounded-lg px-4 py-3"
      elevation="sm"
    >
      <div className="flex size-9 shrink-0 items-center justify-center rounded-md bg-muted">
        <CheckCircle2Icon className="size-4 text-primary" />
      </div>
      <div className="min-w-0 flex-1 text-left">
        <p className="truncate text-sm font-medium">{name}</p>
        <p className="text-xs text-muted-foreground">
          {original} → {compressed}{" "}
          <span className="font-medium text-primary">({savings})</span>
        </p>
      </div>
      <Button variant="outline" size="icon" elevation="sm">
        <DownloadIcon className="size-4" />
      </Button>
    </Card>
  );
}

/** Accepts badge card showing supported formats. */
export function AcceptsCard() {
  return (
    <Card className="h-full space-y-2 p-4" elevation="sm">
      <Text size="sm" weight="semibold">
        Accepts
      </Text>
      <Row gap="sm" wrap>
        {["JPEG", "PNG", "WebP"].map((fmt) => (
          <span
            key={fmt}
            className="inline-flex items-center gap-1 rounded-full bg-muted px-2.5 py-0.5 text-xs font-medium text-muted-foreground"
          >
            <FileIcon className="size-3" />
            {fmt}
          </span>
        ))}
      </Row>
    </Card>
  );
}
