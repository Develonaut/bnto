"use client";

import { useState, useCallback, useMemo } from "react";
import type { Definition } from "@bnto/core";
import { extractProcessingNodes } from "../_utils/extractProcessingNodes";
import { deriveDefaults } from "../_utils/deriveDefaults";

/** File state, parameter state, and processing node extraction for the bento flow. */
export function useBentoParameters(definition: Definition | null | undefined) {
  const [files, setFiles] = useState<File[]>([]);
  const processingNodes = useMemo(
    () => (definition ? extractProcessingNodes(definition) : []),
    [definition],
  );
  const [parameters, setParameters] = useState<Record<string, unknown>>(() =>
    definition ? deriveDefaults(definition) : {},
  );
  const setParameter = useCallback(
    (name: string, value: unknown) => setParameters((prev) => ({ ...prev, [name]: value })),
    [],
  );
  return { files, setFiles, processingNodes, parameters, setParameter };
}
