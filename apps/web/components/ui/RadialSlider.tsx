"use client";

import * as React from "react";

import { cn } from "@/lib/cn";
import { GripVerticalIcon } from "@/components/ui/icons";

/* ── RadialSlider ─────────────────────────────────────────────────────
 * A circular / arc slider with an HTML-based thumb.
 *
 * Supports full 360° rings, half-circles, quarter-arcs, or any segment.
 * Keyboard accessible (arrow keys, Home/End). Touch + pointer drag.
 *
 * The track + progress arc are SVG. The thumb is a real HTML element
 * positioned via CSS `translate`, so it can use our full surface/pressable
 * system (which relies on ::before/::after pseudo elements that SVG
 * doesn't support).
 *
 * Angles use CSS convention: 0° = 12 o'clock, clockwise.
 *
 *   Full ring:     startAngle=0   endAngle=360
 *   Bottom half:   startAngle=90  endAngle=270
 *   NW–NE arc:     startAngle=135 endAngle=225
 *
 * Value range is independent of angle range — min/max map linearly
 * onto the arc defined by startAngle/endAngle.
 * ──────────────────────────────────────────────────────────────────── */

export interface RadialSliderProps {
  /** Minimum value. */
  min: number;
  /** Maximum value. */
  max: number;
  /** Current value (controlled). */
  value: number;
  /** Called when value changes via drag or keyboard. */
  onChange: (value: number) => void;
  /** Increment per keyboard step. Defaults to 1. */
  step?: number;
  /** Where the arc begins (degrees, 0 = top, clockwise). Default 0. */
  startAngle?: number;
  /** Where the arc ends (degrees, 0 = top, clockwise). Default 360. */
  endAngle?: number;
  /** Outer size in px. Default 48. */
  size?: number;
  /** Track + active arc stroke width. Default 4. */
  strokeWidth?: number;
  /** CSS class for track stroke color. Default "text-muted". */
  trackClassName?: string;
  /** CSS class for active arc + thumb color. Default "text-warning". */
  activeClassName?: string;
  /** Hide the filled progress arc (show only track + thumb). */
  hideProgress?: boolean;
  /**
   * Custom thumb element. Receives `isDragging` for pressed states.
   * The element is centered on the track at the current value's angle.
   * Can be a surface/pressable Button — it's real HTML, not SVG.
   *
   * Default: a 12px circle matching `activeClassName`.
   */
  renderThumb?: (props: { isDragging: boolean }) => React.ReactNode;
  /** Content rendered in the center of the dial. */
  children?: React.ReactNode;
  /** Accessible label for screen readers. */
  "aria-label"?: string;
  className?: string;
}

// ── Geometry helpers ──────────────────────────────────────────────────

function toRad(deg: number) {
  return (deg * Math.PI) / 180;
}

/**
 * CSS-convention angle (0° = 12 o'clock, CW) → {x, y} on a circle.
 * Returns coordinates relative to the circle center (0, 0).
 */
function polarToOffset(angleDeg: number, r: number) {
  const rad = toRad(angleDeg - 90);
  return { x: r * Math.cos(rad), y: r * Math.sin(rad) };
}

/** Same as polarToOffset but relative to (cx, cy) for SVG paths. */
function polarToXY(angleDeg: number, r: number, cx: number, cy: number) {
  const { x, y } = polarToOffset(angleDeg, r);
  return { x: cx + x, y: cy + y };
}

/**
 * Build an SVG arc path between two CSS-convention angles.
 * Full-circle rendered as two semicircles (SVG degenerate arc fix).
 */
function arcPath(from: number, to: number, r: number, cx: number, cy: number) {
  let sweep = to - from;
  if (sweep < 0) sweep += 360;
  if (sweep < 0.1) return "";

  if (sweep >= 359.99) {
    const top = polarToXY(from, r, cx, cy);
    const bottom = polarToXY(from + 180, r, cx, cy);
    return [
      `M ${top.x} ${top.y}`,
      `A ${r} ${r} 0 1 1 ${bottom.x} ${bottom.y}`,
      `A ${r} ${r} 0 1 1 ${top.x} ${top.y}`,
    ].join(" ");
  }

  const start = polarToXY(from, r, cx, cy);
  const end = polarToXY(to, r, cx, cy);
  const largeArc = sweep > 180 ? 1 : 0;
  return `M ${start.x} ${start.y} A ${r} ${r} 0 ${largeArc} 1 ${end.x} ${end.y}`;
}

// ── Value ↔ Angle mapping ────────────────────────────────────────────

function valueToAngle(
  value: number,
  min: number,
  max: number,
  startAngle: number,
  endAngle: number,
) {
  const t = Math.max(0, Math.min(1, (value - min) / (max - min)));
  let span = endAngle - startAngle;
  if (span < 0) span += 360;
  return startAngle + t * span;
}

function angleToValue(
  rawAngle: number,
  min: number,
  max: number,
  startAngle: number,
  endAngle: number,
) {
  let span = endAngle - startAngle;
  if (span < 0) span += 360;

  let offset = rawAngle - startAngle;
  if (offset < 0) offset += 360;

  // For partial arcs, clamp when pointer is in the dead zone
  if (span < 360 && offset > span) {
    const pastEnd = offset - span;
    const deadZone = 360 - span;
    offset = pastEnd < deadZone / 2 ? span : 0;
  }

  const t = Math.max(0, Math.min(1, offset / span));
  return Math.round(min + t * (max - min));
}

function pointerToAngle(clientX: number, clientY: number, rect: DOMRect) {
  const cx = rect.left + rect.width / 2;
  const cy = rect.top + rect.height / 2;
  const rad = Math.atan2(clientY - cy, clientX - cx);
  let deg = (rad * 180) / Math.PI + 90;
  if (deg < 0) deg += 360;
  return deg;
}

// ── Component ────────────────────────────────────────────────────────

export function RadialSlider({
  min,
  max,
  value,
  onChange,
  step = 1,
  startAngle = 0,
  endAngle = 360,
  size = 48,
  strokeWidth = 10,
  trackClassName = "text-input",
  activeClassName = "text-primary",
  hideProgress = false,
  renderThumb,
  children,
  "aria-label": ariaLabel,
  className,
}: RadialSliderProps) {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const thumbRef = React.useRef<HTMLDivElement>(null);
  const [isDragging, setIsDragging] = React.useState(false);
  const [isHovering, setIsHovering] = React.useState(false);
  const draggingRef = React.useRef(false);

  // Track radius — inset from the edge to leave room for the thumb
  const trackInset = strokeWidth * 2 + 4;
  const radius = (size - trackInset) / 2;
  const svgCenter = size / 2;

  // Thumb position as offset from center of the container
  const currentAngle = valueToAngle(value, min, max, startAngle, endAngle);
  const thumbOffset = polarToOffset(currentAngle, radius);

  // SVG paths — three layers for partial arcs:
  //   1. Full ring (faint guide showing the complete circle)
  //   2. Arc-range track (slightly stronger, shows the usable range)
  //   3. Active progress (filled portion from start to current value)
  let arcSpan = endAngle - startAngle;
  if (arcSpan < 0) arcSpan += 360;
  const isPartialArc = arcSpan < 360;
  const ringPath = isPartialArc
    ? arcPath(0, 360, radius, svgCenter, svgCenter)
    : "";
  const trackPath = arcPath(startAngle, endAngle, radius, svgCenter, svgCenter);
  const activePath =
    !hideProgress && value > min
      ? arcPath(startAngle, currentAngle, radius, svgCenter, svgCenter)
      : "";

  // ── Pointer handling ───────────────────────────────────────────────

  const updateFromPointer = React.useCallback(
    (clientX: number, clientY: number) => {
      if (!containerRef.current) return;
      const rect = containerRef.current.getBoundingClientRect();
      const angle = pointerToAngle(clientX, clientY, rect);
      onChange(angleToValue(angle, min, max, startAngle, endAngle));
    },
    [min, max, startAngle, endAngle, onChange],
  );

  /** Check if a pointer angle falls within the arc (with a small margin). */
  const isInArc = React.useCallback(
    (clientX: number, clientY: number) => {
      if (!containerRef.current) return false;
      // Full-circle arcs have no dead zone
      let span = endAngle - startAngle;
      if (span < 0) span += 360;
      if (span >= 360) return true;

      const rect = containerRef.current.getBoundingClientRect();
      const angle = pointerToAngle(clientX, clientY, rect);
      let offset = angle - startAngle;
      if (offset < 0) offset += 360;
      // Allow a small margin (20°) past each end for forgiveness
      return offset <= span + 20 || offset >= 360 - 20;
    },
    [startAngle, endAngle],
  );

  const onPointerDown = React.useCallback(
    (e: React.PointerEvent) => {
      // Ignore clicks in the dead zone (outside the arc)
      if (!isInArc(e.clientX, e.clientY)) return;
      draggingRef.current = true;
      setIsDragging(true);
      containerRef.current?.setPointerCapture(e.pointerId);
      updateFromPointer(e.clientX, e.clientY);
    },
    [isInArc, updateFromPointer],
  );

  const onPointerMove = React.useCallback(
    (e: React.PointerEvent) => {
      if (draggingRef.current) {
        updateFromPointer(e.clientX, e.clientY);
      } else if (containerRef.current) {
        // Show pointer cursor only over the active arc area
        const inArc = isInArc(e.clientX, e.clientY);
        containerRef.current.style.cursor = inArc ? "pointer" : "default";

        // Detect hover over thumb for pressable data-hover
        if (thumbRef.current) {
          const rect = thumbRef.current.getBoundingClientRect();
          const dx = e.clientX - (rect.left + rect.width / 2);
          const dy = e.clientY - (rect.top + rect.height / 2);
          setIsHovering(dx * dx + dy * dy <= (rect.width / 2 + 4) ** 2);
        }
      }
    },
    [isInArc, updateFromPointer],
  );

  const onPointerUp = React.useCallback(() => {
    draggingRef.current = false;
    setIsDragging(false);
  }, []);

  // ── Keyboard handling ──────────────────────────────────────────────

  const onKeyDown = React.useCallback(
    (e: React.KeyboardEvent) => {
      let next: number | null = null;
      switch (e.key) {
        case "ArrowRight":
        case "ArrowUp":
          next = Math.min(max, value + step);
          break;
        case "ArrowLeft":
        case "ArrowDown":
          next = Math.max(min, value - step);
          break;
        case "Home":
          next = min;
          break;
        case "End":
          next = max;
          break;
        default:
          return;
      }
      e.preventDefault();
      onChange(next);
    },
    [min, max, value, step, onChange],
  );

  // ── Default thumb ──────────────────────────────────────────────────

  const defaultThumb = (
    <div ref={thumbRef} className="surface surface-primary elevation-sm pressable flex items-center justify-center size-8 rounded-full outline-none! ring-0" data-active={isDragging || undefined} data-hover={isHovering || undefined}>
      <GripVerticalIcon strokeWidth={3} className="size-3.5 shrink-0" />
    </div>
  );

  return (
    <div
      ref={containerRef}
      className={cn(
        "relative touch-none select-none outline-none",
        className,
      )}
      style={{ width: size, height: size }}
      role="slider"
      tabIndex={0}
      aria-valuemin={min}
      aria-valuemax={max}
      aria-valuenow={value}
      aria-label={ariaLabel}
      onPointerDown={onPointerDown}
      onPointerMove={onPointerMove}
      onPointerUp={onPointerUp}
      onPointerLeave={() => setIsHovering(false)}
      onKeyDown={onKeyDown}
    >
      {/* SVG layer: track + progress arc */}
      <svg
        width={size}
        height={size}
        viewBox={`0 0 ${size} ${size}`}
        className="pointer-events-none absolute inset-0"
      >
        {/* Full ring guide — faint circle showing the complete dial */}
        {ringPath && (
          <>
            <path d={ringPath} fill="none" stroke="currentColor" className="text-border" strokeWidth={strokeWidth + 2} strokeLinecap="round" opacity={0.3} />
            <path d={ringPath} fill="none" stroke="currentColor" className={trackClassName} strokeWidth={strokeWidth} strokeLinecap="round" opacity={0.3} />
          </>
        )}
        {/* Arc-range track — border layer + fill layer */}
        {trackPath && (
          <>
            <path d={trackPath} fill="none" stroke="currentColor" className="text-border" strokeWidth={strokeWidth + 2} strokeLinecap="round" />
            <path d={trackPath} fill="none" stroke="currentColor" className={trackClassName} strokeWidth={strokeWidth} strokeLinecap="round" />
          </>
        )}
        {activePath && (
          <path
            d={activePath}
            fill="none"
            stroke="currentColor"
            className={activeClassName}
            strokeWidth={strokeWidth}
            strokeLinecap="round"
          />
        )}
      </svg>

      {/* Center content */}
      {children && (
        <div className="pointer-events-none absolute inset-0 flex items-center justify-center">
          {children}
        </div>
      )}

      {/* HTML thumb — positioned at the current angle on the track.
       * Centered on the track point via -50% translate. */}
      <div
        className="pointer-events-none absolute left-1/2 top-1/2"
        style={{
          transform: `translate(calc(${thumbOffset.x}px - 50%), calc(${thumbOffset.y}px - 50%))`,
        }}
      >
        {renderThumb ? renderThumb({ isDragging }) : defaultThumb}
      </div>
    </div>
  );
}
