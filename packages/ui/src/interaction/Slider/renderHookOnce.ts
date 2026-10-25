/**
 * Test helper: render a hook once on the server and return what it gave back.
 * Enough for hooks built on useMemo/useCallback; @bnto/ui tests run in Node with no DOM.
 */

import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";

export function renderHookOnce<T>(hook: () => T): T {
  let result: { value: T } | undefined;
  function Probe() {
    result = { value: hook() };
    return null;
  }
  renderToStaticMarkup(createElement(Probe));
  if (!result) throw new Error("hook did not run");
  return result.value;
}
