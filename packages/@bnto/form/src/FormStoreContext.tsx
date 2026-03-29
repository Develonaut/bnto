"use client";

/**
 * FormStoreContext — React context + hooks for cross-field form value access.
 *
 * The provider syncs the parent's `values` prop into a Zustand store.
 * Controls subscribe to individual values via `useFormValue("fieldName")`
 * with Zustand selector granularity (re-render only when THAT value changes).
 */

import { createContext, useContext, useEffect, useRef, type ReactNode } from "react";
import { useStore } from "zustand";
import { createFormStore, type FormStore } from "./formStore";

const FormStoreContext = createContext<FormStore | null>(null);

interface FormStoreProviderProps {
  values: Record<string, unknown>;
  onChange: (name: string, value: unknown) => void;
  /** Optional source files (e.g. uploaded images) for controls that show previews. */
  files?: File[];
  children: ReactNode;
}

function FormStoreProvider({ values, onChange, files = [], children }: FormStoreProviderProps) {
  const storeRef = useRef(createFormStore(values, onChange, files));

  // Sync incoming props into the store via useEffect to avoid
  // triggering Zustand subscriber re-renders during this component's render.
  useEffect(() => {
    storeRef.current.setState({ values, onChange, files });
  }, [values, onChange, files]);

  return <FormStoreContext.Provider value={storeRef.current}>{children}</FormStoreContext.Provider>;
}

function useFormStore(): FormStore {
  const store = useContext(FormStoreContext);
  if (!store) throw new Error("useFormValue must be used within SchemaForm");
  return store;
}

/** Read a single form value — only re-renders when THIS value changes. */
function useFormValue(name: string): unknown {
  const store = useFormStore();
  return useStore(store, (s) => s.values[name]);
}

/** Read all form values. */
function useFormValues(): Record<string, unknown> {
  const store = useFormStore();
  return useStore(store, (s) => s.values);
}

/** Get the form onChange handler. */
function useFormOnChange(): (name: string, value: unknown) => void {
  const store = useFormStore();
  return useStore(store, (s) => s.onChange);
}

/** Read the source files passed to the form (e.g. uploaded images for preview). */
function useFormFiles(): File[] {
  const store = useFormStore();
  return useStore(store, (s) => s.files);
}

export { FormStoreProvider, useFormValue, useFormValues, useFormOnChange, useFormFiles };
