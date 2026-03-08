"use client";

import type { ReactNode } from "react";
import { Card } from "../surface/Card";
import { IconBadge } from "../typography/IconBadge";
import { Badge } from "../typography/Badge";
import { Row } from "../layout/Row";
import { Stack } from "../layout/Stack";
import { CheckCircle2Icon } from "../icons";

interface ResultFileCardProps {
  /** Output filename. */
  filename: string;
  /** File extension (e.g., "png"). Shown as uppercase pill badge. */
  extension?: string | null;
  /** Formatted output file size (e.g., "950.2 KB"). */
  outputSize: string;
  /** Formatted original file size (e.g., "1 MB"). Shown with strikethrough when savings exist. */
  originalSize?: string;
  /** Savings percentage string (e.g., "-45%"). Shown in primary color. */
  savings?: string;
  /** Icon element for the badge. Defaults to CheckCircle2. */
  icon?: ReactNode;
  /** Action slot (typically a download button). */
  action?: ReactNode;
  className?: string;
  "data-testid"?: string;
}

/**
 * Result file card — shows a processed file with stats.
 *
 * Layout matches the existing FileCard/BrowserFileRow style:
 *   [IconBadge]  filename  [EXT pill]       [action]
 *                originalSize  -X%  newSize
 */
function ResultFileCard({
  filename,
  extension,
  outputSize,
  originalSize,
  savings,
  icon,
  action,
  className,
  ...rest
}: ResultFileCardProps) {
  const hasSavings = originalSize != null && savings != null;

  return (
    <Card
      elevation="sm"
      className={className}
      role="listitem"
      data-testid={rest["data-testid"] ?? "output-file"}
    >
      <Row className="gap-3 rounded-lg px-4 py-3">
        <Row className="min-w-0 flex-1 gap-3">
          <IconBadge variant="primary" size="lg" aria-hidden="true">
            {icon ?? <CheckCircle2Icon className="size-5" />}
          </IconBadge>
          <Stack className="min-w-0 flex-1 gap-0">
            <Row className="items-center gap-1.5">
              <span className="truncate text-sm font-semibold">{filename}</span>
              {extension && (
                <Badge variant="outline" size="sm" className="shrink-0 uppercase">
                  {extension}
                </Badge>
              )}
            </Row>
            <Row className="items-center gap-1.5">
              {hasSavings ? (
                <>
                  <span className="text-xs text-muted-foreground line-through">{originalSize}</span>
                  <span className="text-xs font-semibold text-primary">{savings}</span>
                  <span className="text-xs text-muted-foreground">{outputSize}</span>
                </>
              ) : (
                <span className="text-xs text-muted-foreground">{outputSize}</span>
              )}
            </Row>
          </Stack>
        </Row>
        {action}
      </Row>
    </Card>
  );
}

export { ResultFileCard };
export type { ResultFileCardProps };
