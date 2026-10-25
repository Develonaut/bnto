"use client";

import { useContext } from "react";
import { FormStoreCtx } from "./FormStoreContext";

/** Read all current form values. */
export function useFormValues(): Record<string, unknown> {
  const ctx = useContext(FormStoreCtx);
  return ctx?.values ?? {};
}
