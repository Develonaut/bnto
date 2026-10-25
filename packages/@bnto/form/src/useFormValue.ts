"use client";

import { useContext } from "react";
import { FormStoreCtx } from "./FormStoreContext";

/** Read a single sibling form value by param name. */
export function useFormValue(name: string): unknown {
  const ctx = useContext(FormStoreCtx);
  return ctx?.values[name];
}
