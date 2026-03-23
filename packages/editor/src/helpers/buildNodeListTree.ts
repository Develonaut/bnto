/**
 * buildNodeListTree — builds a hierarchical list from the flat node array.
 *
 * Preserves original array order (1:1 with canvas), nests children
 * under their parent containers. Top-level nodes (including IO) stay
 * in their original position.
 *
 * When no processing nodes exist, `placeholderIndex` indicates where
 * the "Add a node" placeholder should be inserted (before the last
 * IO node, mirroring the canvas placeholder position).
 */

import type { BentoNode, NodeConfig, NodeConfigs } from "../adapters/types";

interface NodeListEntry {
  node: BentoNode;
  config: NodeConfig;
  children: NodeListEntry[];
  isContainer: boolean;
}

interface NodeListTree {
  entries: NodeListEntry[];
  isEmpty: boolean;
  placeholderIndex: number;
}

function buildNodeListTree(nodes: BentoNode[], configs: NodeConfigs): NodeListTree {
  const topLevel: BentoNode[] = [];
  const childrenByParent = new Map<string, BentoNode[]>();
  let hasProcessingNode = false;

  for (const node of nodes) {
    const config = configs[node.id];
    if (!config) continue;

    const parentId = node.data.parentContainerId;
    if (parentId) {
      const siblings = childrenByParent.get(parentId);
      if (siblings) {
        siblings.push(node);
      } else {
        childrenByParent.set(parentId, [node]);
      }
    } else {
      topLevel.push(node);
      if (!node.data.isIoNode) hasProcessingNode = true;
    }
  }

  function buildEntry(node: BentoNode): NodeListEntry {
    const config = configs[node.id]!;
    const isContainer = node.data.isContainer ?? false;
    const children = isContainer ? (childrenByParent.get(node.id) ?? []).map(buildEntry) : [];
    return { node, config, children, isContainer };
  }

  const entries = topLevel.map(buildEntry);
  const isEmpty = !hasProcessingNode;

  let placeholderIndex = -1;
  if (isEmpty && entries.length > 0) {
    const lastIoIndex = findLastIndex(entries, (e) => e.node.data.isIoNode === true);
    placeholderIndex = lastIoIndex >= 0 ? lastIoIndex : entries.length;
  }

  return { entries, isEmpty, placeholderIndex };
}

function findLastIndex<T>(arr: T[], predicate: (item: T) => boolean): number {
  for (let i = arr.length - 1; i >= 0; i--) {
    if (predicate(arr[i]!)) return i;
  }
  return -1;
}

export { buildNodeListTree };
export type { NodeListEntry, NodeListTree };
