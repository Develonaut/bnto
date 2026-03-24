/**
 * resolveFocusZone — pure function resolving focus transitions
 * between the tree zone and the command input zone.
 *
 * Keyboard model (from cmd-editor.md):
 *   Tab (tree)           → focus command input
 *   / or Cmd+K (tree)    → focus command input
 *   Escape (command)     → focus tree (only when input is empty)
 */

type FocusZone = "tree" | "command";

interface FocusEvent {
  key: string;
  metaKey: boolean;
  /** Current value of the command input (for Escape guard). */
  inputValue: string;
}

interface FocusTransition {
  target: FocusZone;
}

function resolveFocusZone(currentZone: FocusZone, event: FocusEvent): FocusTransition | null {
  if (currentZone === "tree") {
    if (event.key === "Tab") return { target: "command" };
    if (event.key === "/") return { target: "command" };
    if (event.key === "k" && event.metaKey) return { target: "command" };
  }

  if (currentZone === "command") {
    if (event.key === "Escape" && event.inputValue === "") {
      return { target: "tree" };
    }
  }

  return null;
}

export { resolveFocusZone };
export type { FocusZone, FocusEvent, FocusTransition };
