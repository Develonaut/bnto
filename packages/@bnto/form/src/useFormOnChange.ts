"use client";

import { useContext } from "react";
import { FormStoreCtx } from "./FormStoreContext";

const NOOP_ON_CHANGE = () => {};

/** Get the form-level onChange handler for setting sibling field values. */
export function useFormOnChange(): (name: string, value: unknown) => void {
  const ctx = useContext(FormStoreCtx);
  return ctx?.onChange ?? NOOP_ON_CHANGE;
}
