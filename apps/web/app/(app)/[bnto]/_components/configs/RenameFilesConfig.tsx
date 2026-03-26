"use client";

import type { ChangeEvent } from "react";
import { useCallback } from "react";
import { Input, Label } from "@bnto/ui";
import type { RenameFilesConfig as Config } from "./types";
import { RenamePatternPreview } from "./RenamePatternPreview";

interface RenameFilesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function RenameFilesConfig({ value, onChange }: RenameFilesConfigProps) {
  const handlePatternChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => onChange({ ...value, pattern: e.target.value }),
    [onChange, value],
  );

  return (
    <div className="flex w-full flex-col gap-1">
      <Label htmlFor="rename-pattern" className="text-muted-foreground text-xs">
        Pattern
      </Label>
      <div className="flex items-center gap-3">
        <Input
          id="rename-pattern"
          type="text"
          wrapperClassName="w-full"
          aria-describedby="rename-pattern-help"
          value={value.pattern}
          onChange={handlePatternChange}
          placeholder="renamed-{{name}}"
        />
        <RenamePatternPreview pattern={value.pattern} />
      </div>
      <p id="rename-pattern-help" className="text-muted-foreground text-xs">
        Use <span className="font-mono">{"{{name}}"}</span> and{" "}
        <span className="font-mono">{"{{ext}}"}</span> as placeholders
      </p>
    </div>
  );
}
