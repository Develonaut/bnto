import type AppStrings from "./en.json";
import type NodeStrings from "./generated/nodes.json";

/** Recursively derive dot-path keys from a nested JSON object type. */
type Join<K, P> = K extends string ? (P extends string ? `${K}.${P}` : never) : never;

type Paths<T> = T extends object
  ? {
      [K in keyof T]: T[K] extends string ? K & string : Join<K, Paths<T[K]>>;
    }[keyof T]
  : never;

type AppKey = Paths<typeof AppStrings>;
type NodeKey = Paths<typeof NodeStrings>;

/** Union of all valid dot-path string keys across both JSON files. */
export type StringKey = AppKey | NodeKey;
