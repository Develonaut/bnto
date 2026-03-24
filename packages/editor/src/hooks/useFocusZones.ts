/**
 * useFocusZones — manages focus flow between tree and command zones.
 *
 * Two zones: "tree" (the node list) and "command" (the cmdk input).
 * Keyboard shortcuts move focus between them:
 *   Tab, /, Cmd+K → tree → command
 *   Escape (empty) → command → tree
 *
 * Returns refs for each zone and an onKeyDown handler for the shell
 * that intercepts zone-transition keys before they bubble.
 */

"use client";

import { useCallback, useRef } from "react";
import type { KeyboardEvent, RefObject } from "react";
import { resolveFocusZone } from "../actions/resolveFocusZone";
import type { FocusZone } from "../actions/resolveFocusZone";

interface UseFocusZonesResult {
  treeRef: RefObject<HTMLDivElement | null>;
  commandRef: RefObject<HTMLDivElement | null>;
  onShellKeyDown: (e: KeyboardEvent) => void;
}

function useFocusZones(): UseFocusZonesResult {
  const treeRef = useRef<HTMLDivElement | null>(null);
  const commandRef = useRef<HTMLDivElement | null>(null);

  const onShellKeyDown = useCallback((e: KeyboardEvent) => {
    const currentZone = detectCurrentZone(e.target as HTMLElement, treeRef, commandRef);
    if (!currentZone) return;

    const inputValue = getCommandInputValue(commandRef);

    const transition = resolveFocusZone(currentZone, {
      key: e.key,
      metaKey: e.metaKey,
      inputValue,
    });

    if (!transition) return;

    e.preventDefault();
    e.stopPropagation();
    focusZone(transition.target, treeRef, commandRef);
  }, []);

  return { treeRef, commandRef, onShellKeyDown };
}

/** Determine which zone the event target belongs to. */
function detectCurrentZone(
  target: HTMLElement,
  treeRef: RefObject<HTMLDivElement | null>,
  commandRef: RefObject<HTMLDivElement | null>,
): FocusZone | null {
  if (treeRef.current?.contains(target)) return "tree";
  if (commandRef.current?.contains(target)) return "command";
  return null;
}

/** Read the command input's current value from the DOM. */
function getCommandInputValue(commandRef: RefObject<HTMLDivElement | null>): string {
  const input = commandRef.current?.querySelector<HTMLInputElement>("[data-slot='command-input']");
  return input?.value ?? "";
}

/** Move DOM focus to the target zone. */
function focusZone(
  target: FocusZone,
  treeRef: RefObject<HTMLDivElement | null>,
  commandRef: RefObject<HTMLDivElement | null>,
): void {
  if (target === "tree") {
    // Focus the active treeitem (roving tabindex) or first treeitem
    const active =
      treeRef.current?.querySelector<HTMLElement>("[role='treeitem'][tabindex='0']") ??
      treeRef.current?.querySelector<HTMLElement>("[role='treeitem']");
    active?.focus();
  } else {
    // Focus the cmdk input element
    const input = commandRef.current?.querySelector<HTMLInputElement>(
      "[data-slot='command-input']",
    );
    input?.focus();
  }
}

export { useFocusZones };
export type { UseFocusZonesResult };
