"use client";

import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { RenameFilesConfig as Config } from "./types";

interface RenameFilesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function RenameFilesConfig({
  value,
  onChange,
}: RenameFilesConfigProps) {
  return (
    <div className="space-y-4">
      <div className="space-y-2">
        <Label htmlFor="rename-pattern">Rename pattern</Label>
        <Input
          id="rename-pattern"
          type="text"
          value={value.pattern}
          onChange={(e) => onChange({ ...value, pattern: e.target.value })}
          placeholder="renamed-{{name}}"
        />
        <p className="text-muted-foreground text-xs">
          Use <code className="bg-muted rounded px-1 font-mono">{"{{name}}"}</code>{" "}
          for the original filename and{" "}
          <code className="bg-muted rounded px-1 font-mono">{"{{ext}}"}</code>{" "}
          for the extension.
        </p>
      </div>

      {value.pattern && (
        <div className="rounded-md border border-border bg-muted/50 px-3 py-2">
          <p className="text-muted-foreground text-xs">
            <span className="font-medium">Preview:</span>{" "}
            <span className="font-mono">
              {value.pattern
                .replace("{{name}}", "photo")
                .replace("{{ext}}", ".png")}
            </span>
          </p>
        </div>
      )}
    </div>
  );
}
