"use client";

import { createContext, useContext } from "react";

interface FormControlContextValue {
  id: string;
  helpTextId: string;
}

export const FormControlContext = createContext<FormControlContextValue | null>(null);

/** Read FormControl context. Returns `null` when not inside a FormControl. */
export function useFormControlContext() {
  return useContext(FormControlContext);
}
