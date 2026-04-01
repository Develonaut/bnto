"use client";

import { useCallback, useMemo } from "react";
import type { ControlProps } from "./types";
import { useFormFiles, useFormOnChange, useFormValues } from "../FormStoreContext";
import { POSITIONS, imageAreaStyle, overlayPositionStyle } from "./watermarkLayout";
import { useSourcePreview } from "./useSourcePreview";

function WatermarkPreviewControl({ id, value, onChange }: ControlProps) {
  const values = useFormValues();
  const files = useFormFiles();
  const formOnChange = useFormOnChange();
  const { sourceUrl, aspectRatio } = useSourcePreview(files);

  const selected = typeof value === "string" ? value : "bottom-right";
  const watermarkDataUri = typeof values.overlay === "string" ? values.overlay : "";
  const size = typeof values.size === "number" ? values.size : 25;
  const opacity = typeof values.opacity === "number" ? values.opacity : 80;
  const offsetX = typeof values.offsetX === "number" ? values.offsetX : 0;
  const offsetY = typeof values.offsetY === "number" ? values.offsetY : 0;

  const handlePositionChange = useCallback(
    (pos: string) => {
      onChange(pos);
      formOnChange("offsetX", 0);
      formOnChange("offsetY", 0);
    },
    [onChange, formOnChange],
  );

  const areaStyle = useMemo(() => imageAreaStyle(aspectRatio), [aspectRatio]);
  const overlayStyle = useMemo(
    () => ({ ...overlayPositionStyle(selected, size, offsetX, offsetY), opacity: opacity / 100 }),
    [selected, size, offsetX, offsetY, opacity],
  );

  return (
    <div
      className="sticky top-0 z-30"
      data-testid="watermark-preview"
      data-aspect-ratio={aspectRatio}
    >
      <PreviewGrid
        id={id}
        sourceUrl={sourceUrl}
        areaStyle={areaStyle}
        selected={selected}
        onPositionChange={handlePositionChange}
        watermarkDataUri={watermarkDataUri}
        overlayStyle={overlayStyle}
      />
    </div>
  );
}

interface PreviewGridProps {
  id: string;
  sourceUrl: string;
  areaStyle: React.CSSProperties;
  selected: string;
  onPositionChange: (pos: string) => void;
  watermarkDataUri: string;
  overlayStyle: React.CSSProperties;
}

function PreviewGrid({
  id,
  sourceUrl,
  areaStyle,
  selected,
  onPositionChange,
  watermarkDataUri,
  overlayStyle,
}: PreviewGridProps) {
  return (
    <div className="relative w-full aspect-square rounded-lg overflow-hidden bg-muted border border-border">
      <div className="absolute" style={areaStyle}>
        {sourceUrl && (
          <img
            src={sourceUrl}
            alt="Source preview"
            className="absolute inset-0 w-full h-full object-cover"
          />
        )}
        <div
          className="absolute inset-0 grid grid-cols-3 grid-rows-3"
          role="radiogroup"
          aria-label="Watermark position"
          data-testid={`control-position-grid-${id}`}
        >
          {POSITIONS.map((pos) => (
            <PositionDot
              key={pos}
              pos={pos}
              selected={selected === pos}
              hasImage={!!sourceUrl}
              onClick={onPositionChange}
            />
          ))}
        </div>
        {watermarkDataUri && (
          <img
            src={watermarkDataUri}
            alt="Overlay preview"
            className="pointer-events-none"
            style={overlayStyle}
            data-testid="watermark-preview-overlay"
          />
        )}
      </div>
    </div>
  );
}

const DOT_BASE = "size-3 rounded-full transition-colors hover:scale-125";
const DOT_ACTIVE = `${DOT_BASE} bg-primary ring-2 ring-primary/30`;
const DOT_ON_IMAGE = `${DOT_BASE} bg-white/70 shadow-sm`;
const DOT_ON_BLANK = `${DOT_BASE} border-2 border-border`;

function PositionDot({
  pos,
  selected,
  hasImage,
  onClick,
}: {
  pos: string;
  selected: boolean;
  hasImage: boolean;
  onClick: (p: string) => void;
}) {
  const handleClick = useCallback(() => onClick(pos), [onClick, pos]);
  const dotClass = selected ? DOT_ACTIVE : hasImage ? DOT_ON_IMAGE : DOT_ON_BLANK;
  return (
    <button
      type="button"
      role="radio"
      aria-checked={selected}
      aria-label={pos.replace(/-/g, " ")}
      onClick={handleClick}
      className="flex items-center justify-center"
      data-testid={`position-${pos}`}
    >
      <span className={dotClass} />
    </button>
  );
}

export { WatermarkPreviewControl };
