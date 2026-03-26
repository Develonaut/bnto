/** Selector: can the pipeline be run? False if already running or no processing nodes. */

import type { EditorState } from "../../store/types";

function selectCanRun(s: EditorState): boolean {
  if (s.executionPhase === "running") return false;
  return s.nodes.some((n) => {
    const config = s.configs[n.id];
    return config && config.nodeType !== "input" && config.nodeType !== "output";
  });
}

export { selectCanRun };
