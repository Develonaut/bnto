"use client";

import { useContext } from "react";
import { FormStoreCtx } from "./FormStoreContext";

/** Read the input files provided to the form. */
export function useFormFiles(): File[] {
  const ctx = useContext(FormStoreCtx);
  return ctx?.files ?? [];
}
