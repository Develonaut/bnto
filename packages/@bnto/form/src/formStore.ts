/**
 * Form store factory — Zustand vanilla store for cross-field communication.
 *
 * Mirrors the editor's `values` prop into a Zustand store so any control
 * can subscribe to sibling values via `useFormValue()`. The store is a
 * read bridge — writes still go through the existing `onChange` callback.
 */

import { createStore } from "zustand/vanilla";

interface FormState {
  values: Record<string, unknown>;
  onChange: (name: string, value: unknown) => void;
}

type FormStore = ReturnType<typeof createFormStore>;

function createFormStore(
  values: Record<string, unknown>,
  onChange: (name: string, value: unknown) => void,
) {
  return createStore<FormState>(() => ({ values, onChange }));
}

export { createFormStore };
export type { FormState, FormStore };
