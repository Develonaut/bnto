import { Children, isValidElement } from "react";
import type { ReactNode, ReactElement, Key } from "react";

import type { CellLayout } from "./bentoGridContext";
import { assignCellLayouts } from "./assignCellLayouts";
import { assignWithPinned } from "./assignWithPinned";

interface PinnedProps {
  colSpan: 1 | 2 | 3;
  rowSpan: 1 | 2;
  featured?: boolean;
  children: ReactNode;
}

export interface BentoEntry {
  layout: CellLayout;
  element: ReactNode;
  key: Key;
}

function isPinned(child: ReactNode): child is ReactElement<PinnedProps> {
  return isValidElement(child) && (child.type as { displayName?: string }).displayName === "BentoGrid.Pinned";
}

/** Split children into pinned + flow, compute layouts for each. */
export function partitionBentoChildren(
  children: ReactNode,
  cols: number,
  uniform: boolean,
): { entries: BentoEntry[]; rows: number } {
  const childArray = Children.toArray(children);

  const pinned: { layout: CellLayout; element: ReactNode; key: Key }[] = [];
  const flow: { element: ReactNode; key: Key }[] = [];

  childArray.forEach((child, i) => {
    const key = isValidElement(child) ? child.key ?? i : i;
    if (isPinned(child)) {
      pinned.push({
        layout: {
          colSpan: child.props.colSpan,
          rowSpan: child.props.rowSpan,
          featured: child.props.featured ?? true,
        },
        element: child.props.children,
        key,
      });
    } else {
      flow.push({ element: child, key });
    }
  });

  if (pinned.length > 0) {
    return resolvePinnedLayout(pinned, flow, cols);
  }

  if (uniform) {
    return resolveUniformLayout(childArray, cols);
  }

  return resolveAutoLayout(childArray, cols);
}

function resolvePinnedLayout(
  pinned: BentoEntry[],
  flow: { element: ReactNode; key: Key }[],
  cols: number,
) {
  const { flowLayouts, rows } = assignWithPinned(
    pinned.map((p) => p.layout),
    flow.length,
    cols,
  );
  return {
    entries: [
      ...pinned,
      ...flow.map((f, i) => ({ ...f, layout: flowLayouts[i] })),
    ],
    rows,
  };
}

function resolveUniformLayout(childArray: ReactNode[], cols: number) {
  const layout: CellLayout = { colSpan: 1, rowSpan: 1, featured: false };
  return {
    entries: childArray.map((child, i) => ({
      layout,
      element: child,
      key: isValidElement(child) ? child.key ?? i : i,
    })),
    rows: Math.ceil(childArray.length / cols),
  };
}

function resolveAutoLayout(childArray: ReactNode[], cols: number) {
  const result = assignCellLayouts(childArray.length, cols);
  return {
    entries: childArray.map((child, i) => ({
      layout: result.layouts[i],
      element: child,
      key: isValidElement(child) ? child.key ?? i : i,
    })),
    rows: result.rows,
  };
}
