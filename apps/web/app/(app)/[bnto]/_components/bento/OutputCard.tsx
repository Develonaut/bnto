"use client";

import { Card, CardContent, Text, Stack } from "@bnto/ui";
import type { BrowserFileResult } from "@bnto/core";
import { OutputResultRow } from "./OutputResultRow";

interface OutputCardProps {
  results: BrowserFileResult[];
  onDownload: (result: BrowserFileResult) => void;
}

/** Download results per file. */
export function OutputCard({ results, onDownload }: OutputCardProps) {
  if (results.length === 0) {
    return (
      <Card elevation="sm" className="flex items-center justify-center p-5">
        <Text size="sm" color="muted">
          Results will appear here
        </Text>
      </Card>
    );
  }

  return (
    <Card elevation="sm" className="p-5">
      <CardContent className="p-0">
        <Text size="xs" color="muted" className="mb-3 font-medium uppercase tracking-wider">
          Output
        </Text>
        <Stack gap="sm">
          {results.map((result, i) => (
            <OutputResultRow
              key={`${result.filename}-${i}`}
              result={result}
              onDownload={onDownload}
            />
          ))}
        </Stack>
      </CardContent>
    </Card>
  );
}
