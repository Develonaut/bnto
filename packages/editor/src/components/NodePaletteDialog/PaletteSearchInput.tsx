import { useCallback, type ChangeEvent } from "react";
import { Input, SearchIcon } from "@bnto/ui";

interface PaletteSearchInputProps {
  value: string;
  onChange: (value: string) => void;
}

function PaletteSearchInput({ value, onChange }: PaletteSearchInputProps) {
  const handleChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => onChange(e.target.value),
    [onChange],
  );

  return (
    <div className="relative mb-3">
      <SearchIcon className="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
      <Input
        placeholder="Search nodes..."
        value={value}
        onChange={handleChange}
        className="pl-9"
        data-testid="node-palette-search"
        autoFocus
      />
    </div>
  );
}

export { PaletteSearchInput };
