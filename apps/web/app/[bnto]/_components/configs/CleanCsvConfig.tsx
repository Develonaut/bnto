"use client";

import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import type { CleanCsvConfig as Config } from "./types";

interface CleanCsvConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

const OPTIONS = [
  {
    id: "trimWhitespace" as const,
    label: "Trim whitespace",
    description: "Remove leading and trailing spaces from all cells",
  },
  {
    id: "removeEmptyRows" as const,
    label: "Remove empty rows",
    description: "Delete rows where all cells are empty",
  },
  {
    id: "removeDuplicates" as const,
    label: "Remove duplicates",
    description: "Delete rows that are exact duplicates of another row",
  },
];

export function CleanCsvConfig({ value, onChange }: CleanCsvConfigProps) {
  return (
    <div className="space-y-4">
      {OPTIONS.map((opt) => (
        <div key={opt.id} className="flex items-start justify-between gap-4">
          <div className="space-y-0.5">
            <Label htmlFor={`clean-${opt.id}`}>{opt.label}</Label>
            <p className="text-muted-foreground text-xs">{opt.description}</p>
          </div>
          <Switch
            id={`clean-${opt.id}`}
            checked={value[opt.id]}
            onCheckedChange={(checked) =>
              onChange({ ...value, [opt.id]: !!checked })
            }
          />
        </div>
      ))}
    </div>
  );
}
