"use client";

import { Label } from "@/components/ui/Label";
import { Switch } from "@/components/ui/Switch";
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
    <div className="flex w-full items-end gap-6">
      {OPTIONS.map((opt) => (
        <div key={opt.id} className="flex shrink-0 flex-col gap-1">
          <Label htmlFor={`clean-${opt.id}`} className="text-muted-foreground text-xs">{opt.label}</Label>
          <Switch
            id={`clean-${opt.id}`}
            checked={value[opt.id]}
            onCheckedChange={(checked) =>
              onChange({ ...value, [opt.id]: !!checked })
            }
          />
          <p className="text-muted-foreground max-w-24 text-xs leading-tight">{opt.description}</p>
        </div>
      ))}
    </div>
  );
}
