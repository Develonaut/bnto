/**
 * Window-based virtualizer for rendering large lists/grids efficiently.
 *
 * Wraps @tanstack/react-virtual so consumers import from @bnto/ui.
 * If the underlying library changes, only this file needs updating.
 */

export { useWindowVirtualizer } from "@tanstack/react-virtual";
export type { VirtualItem } from "@tanstack/react-virtual";
