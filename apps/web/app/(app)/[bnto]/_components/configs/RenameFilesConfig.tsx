"use client";

import { Input, Label } from "@bnto/ui";
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
    <div className="flex w-full flex-col gap-1">
      <Label htmlFor="rename-pattern" className="text-muted-foreground text-xs">Pattern</Label>
      <div className="flex items-center gap-3">
        <Input
          id="rename-pattern"
          type="text"
          wrapperClassName="w-full"
          aria-describedby="rename-pattern-help"
          value={value.pattern}
          onChange={(e) => onChange({ ...value, pattern: e.target.value })}
          placeholder="renamed-{{name}}"
        />
        {value.pattern && (
          <span className="text-muted-foreground shrink-0 text-xs">
            <span className="font-mono">
              {value.pattern
                .replace("{{name}}", "photo")
                .replace("{{ext}}", ".png")}
            </span>
          </span>
        )}
      </div>
      <p id="rename-pattern-help" className="text-muted-foreground text-xs">
        Use <span className="font-mono">{"{{name}}"}</span> and <span className="font-mono">{"{{ext}}"}</span> as placeholders
      </p>
    </div>
  );
}
