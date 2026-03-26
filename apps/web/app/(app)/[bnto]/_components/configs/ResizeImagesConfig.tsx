"use client";

import { Input, Label, Switch } from "@bnto/ui";
import type { ResizeImagesConfig as Config } from "./types";
import { useResizeImagesHandlers } from "./useResizeImagesHandlers";

interface ResizeImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

export function ResizeImagesConfig({ value, onChange }: ResizeImagesConfigProps) {
  const { handleWidthChange, handleAspectRatioChange } = useResizeImagesHandlers(value, onChange);

  return (
    <div className="flex w-full items-end gap-4">
      <ResizeWidthInput value={value.width} onChange={handleWidthChange} />
      <ResizeAspectRatioSwitch
        value={value.maintainAspectRatio}
        onChange={handleAspectRatioChange}
      />
    </div>
  );
}

function ResizeWidthInput({
  value,
  onChange,
}: {
  value: number;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
}) {
  return (
    <div className="flex shrink-0 flex-col gap-1">
      <Label htmlFor="resize-width" className="text-muted-foreground text-xs">
        Width (px)
      </Label>
      <Input
        id="resize-width"
        type="number"
        min={1}
        max={10000}
        aria-describedby="resize-width-help"
        value={value}
        wrapperClassName="w-24"
        onChange={onChange}
      />
      <p id="resize-width-help" className="text-muted-foreground text-xs">
        Target width in pixels
      </p>
    </div>
  );
}

function ResizeAspectRatioSwitch({
  value,
  onChange,
}: {
  value: boolean;
  onChange: (checked: boolean) => void;
}) {
  return (
    <div className="flex shrink-0 flex-col gap-1">
      <Label htmlFor="resize-aspect-ratio" className="text-muted-foreground text-xs">
        Aspect ratio
      </Label>
      <Switch
        id="resize-aspect-ratio"
        aria-describedby="resize-aspect-ratio-help"
        checked={value}
        onCheckedChange={onChange}
      />
      <p id="resize-aspect-ratio-help" className="text-muted-foreground text-xs">
        Keep proportions
      </p>
    </div>
  );
}
