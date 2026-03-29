"use client";

import { useCallback, useMemo, useRef, useState } from "react";
import { Button, FileUpIcon, FormControl, FormLabel, Input, Row, Slider, XIcon } from "@bnto/ui";
import type { WatermarkImagesConfig as Config } from "./types";
import { useConfigChange } from "./useConfigChange";

const POSITION_GRID: readonly string[][] = [
  ["top-left", "top-center", "top-right"],
  ["middle-left", "center", "middle-right"],
  ["bottom-left", "bottom-center", "bottom-right"],
];

const QUALITY_PRESETS = [
  { value: 60, label: "Draft" },
  { value: 80, label: "Balanced" },
  { value: 100, label: "Maximum" },
];

interface WatermarkImagesConfigProps {
  value: Config;
  onChange: (config: Config) => void;
}

function readFileAsBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result as string);
    reader.onerror = () => reject(new Error("Failed to read file"));
    reader.readAsDataURL(file);
  });
}

function formatLabel(pos: string) {
  return pos
    .split("-")
    .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
    .join(" ");
}

function WatermarkFileInput({ value, onChange }: { value: string; onChange: (v: string) => void }) {
  const inputRef = useRef<HTMLInputElement>(null);
  const [fileName, setFileName] = useState("");

  const handleFileChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (!file) return;
      setFileName(file.name);
      onChange(await readFileAsBase64(file));
    },
    [onChange],
  );

  const handleClear = useCallback(() => {
    onChange("");
    setFileName("");
    if (inputRef.current) inputRef.current.value = "";
  }, [onChange]);

  const handleChoose = useCallback(() => inputRef.current?.click(), []);
  const hasValue = value.length > 0;

  return (
    <FormControl>
      <FormLabel>Watermark Image</FormLabel>
      <input
        ref={inputRef}
        type="file"
        accept="image/jpeg,image/png,image/webp"
        onChange={handleFileChange}
        className="hidden"
        data-testid="watermark-file-input"
      />
      <Row className="gap-2">
        <Input
          readOnly
          value={fileName}
          placeholder="No file chosen"
          wrapperClassName="min-w-0 flex-1"
          onClick={handleChoose}
          className="cursor-pointer"
        />
        {hasValue ? (
          <Button type="button" variant="outline" size="icon" onClick={handleClear} aria-label="Remove file">
            <XIcon />
          </Button>
        ) : (
          <Button type="button" variant="outline" size="icon" onClick={handleChoose} aria-label="Choose file">
            <FileUpIcon />
          </Button>
        )}
      </Row>
    </FormControl>
  );
}

function PositionDot({ pos, selected, onClick }: { pos: string; selected: boolean; onClick: (e: React.MouseEvent<HTMLButtonElement>) => void }) {
  return (
    <button
      type="button"
      role="radio"
      aria-checked={selected}
      aria-label={formatLabel(pos)}
      data-pos={pos}
      onClick={onClick}
      className={`size-4 rounded-full transition-colors ${
        selected ? "bg-primary" : "bg-[var(--surface-muted-wall)] hover:bg-muted-foreground/50"
      }`}
    />
  );
}

function PositionGrid({ value, onChange }: { value: string; onChange: (pos: string) => void }) {
  const handleClick = useCallback(
    (e: React.MouseEvent<HTMLButtonElement>) => {
      const pos = e.currentTarget.dataset.pos;
      if (pos) onChange(pos);
    },
    [onChange],
  );

  return (
    <FormControl>
      <FormLabel>Position</FormLabel>
      <div
        role="radiogroup"
        aria-label="Position"
        data-testid="position-grid"
        className="grid aspect-square w-full grid-cols-3 place-items-center rounded-md border border-[var(--surface-muted-wall)] bg-input p-3"
      >
        {POSITION_GRID.flat().map((pos) => (
          <PositionDot key={pos} pos={pos} selected={pos === value} onClick={handleClick} />
        ))}
      </div>
      <span className="text-center text-xs text-muted-foreground">{formatLabel(value)}</span>
    </FormControl>
  );
}

/** Wraps a numeric config field as a single-element slider tuple. */
function useSliderField(value: number, change: ReturnType<typeof useConfigChange<Config>>, field: keyof Config) {
  const tuple = useMemo(() => [value], [value]);
  const handler = useCallback(([v]: number[]) => change(field, (v ?? value) as Config[typeof field]), [change, field, value]);
  return { value: tuple, onValueChange: handler };
}

export function WatermarkImagesConfig({ value, onChange }: WatermarkImagesConfigProps) {
  const change = useConfigChange(value, onChange);

  const handleWatermark = useCallback((v: string) => change("watermark", v), [change]);
  const handlePosition = useCallback((p: string) => change("position", p), [change]);

  const size = useSliderField(value.size, change, "size");
  const opacity = useSliderField(value.opacity, change, "opacity");
  const offsetX = useSliderField(value.offsetX, change, "offsetX");
  const offsetY = useSliderField(value.offsetY, change, "offsetY");
  const quality = useSliderField(value.quality, change, "quality");

  return (
    <div className="flex w-full flex-col gap-3">
      <WatermarkFileInput value={value.watermark} onChange={handleWatermark} />
      <PositionGrid value={value.position} onChange={handlePosition} />
      <Slider label="Size (%)" {...size} min={1} max={500} />
      <Slider label="Opacity (%)" {...opacity} min={0} max={100} />
      <Slider label="Horizontal Offset (px)" {...offsetX} min={-500} max={500} />
      <Slider label="Vertical Offset (px)" {...offsetY} min={-500} max={500} />
      <Slider label="Quality" {...quality} min={1} max={100} presets={QUALITY_PRESETS} />
    </div>
  );
}
