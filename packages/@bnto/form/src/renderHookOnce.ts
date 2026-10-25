/**
 * Test helper: render a hook once on the server, inside a FormStoreProvider when
 * a store is given, and return what it gave back. @bnto/form tests run in Node
 * with no DOM.
 */

import { createElement } from "react";
import type { ComponentProps } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { FormStoreProvider } from "./FormStoreContext";

type FormStore = Omit<ComponentProps<typeof FormStoreProvider>, "children">;

export function renderHookOnce<T>(hook: () => T, store?: FormStore): T {
  let result: { value: T } | undefined;
  function Probe() {
    result = { value: hook() };
    return null;
  }
  const probe = createElement(Probe);
  renderToStaticMarkup(
    store ? createElement(FormStoreProvider, { ...store, children: probe }) : probe,
  );
  if (!result) throw new Error("hook did not run");
  return result.value;
}
