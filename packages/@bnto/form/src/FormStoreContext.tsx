"use client";

import { createContext, useMemo } from "react";

/**
 * FormStoreContext — provides cross-field value access for controls.
 *
 * Controls like WatermarkPreviewControl need to read sibling field values
 * (watermark image, position, opacity) to render a live preview. This
 * context passes the full values map + files so any control can peek at
 * siblings without prop drilling. Each hook that reads it has its own file:
 * `useFormValue`, `useFormValues`, `useFormFiles`, `useFormOnChange`.
 */

interface FormStoreValue {
  /** Current form parameter values. */
  values: Record<string, unknown>;
  /** Input files from the user (source images for preview). */
  files: File[];
  /** Change handler for setting sibling field values (e.g., reset offsets on position change). */
  onChange: (name: string, value: unknown) => void;
}

const FormStoreCtx = createContext<FormStoreValue | null>(null);

interface FormStoreProviderProps {
  values: Record<string, unknown>;
  files: File[];
  onChange: (name: string, value: unknown) => void;
  children: React.ReactNode;
}

function FormStoreProvider({ values, files, onChange, children }: FormStoreProviderProps) {
  const ctx = useMemo(() => ({ values, files, onChange }), [values, files, onChange]);
  return <FormStoreCtx.Provider value={ctx}>{children}</FormStoreCtx.Provider>;
}

export { FormStoreProvider, FormStoreCtx };
export type { FormStoreValue };
