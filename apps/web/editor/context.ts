/**
 * Editor context — provides the Zustand store to the React tree.
 *
 * The store is created per editor mount (factory pattern) and passed
 * through context so hooks can subscribe without prop drilling.
 */

import { createContext } from "react";
import type { StoreApi } from "zustand";
import type { EditorStore } from "./store/types";

/** Context holding the editor store instance. Null = no provider. */
const EditorContext = createContext<StoreApi<EditorStore> | null>(null);

export { EditorContext };
