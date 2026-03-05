"use client";

import { useCallback } from "react";
import type { NodeParamMeta } from "@bnto/nodes";
import {
  Input,
  Label,
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
  Switch,
  Text,
} from "@bnto/ui";

/**
 * ParameterField — auto-generated form field from schema metadata.
 *
 * Renders the appropriate control based on the Zod-inferred field type.
 * UI metadata (label, description, placeholder) comes from NodeParamMeta.
 */

interface ParameterFieldProps {
  name: string;
  meta: NodeParamMeta;
  type: "string" | "number" | "boolean" | "enum";
  value: unknown;
  required: boolean;
  enumValues?: readonly string[];
  min?: number;
  max?: number;
  onChange: (name: string, value: unknown) => void;
}

function ParameterField({
  name,
  meta,
  type,
  value,
  required,
  enumValues,
  min,
  max,
  onChange,
}: ParameterFieldProps) {
  const handleChange = useCallback(
    (newValue: unknown) => onChange(name, newValue),
    [name, onChange],
  );

  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={`param-${name}`} className="text-xs font-medium">
        {meta.label}
        {required && <span className="ml-0.5 text-destructive">*</span>}
      </Label>

      {type === "enum" && enumValues ? (
        <Select value={String(value ?? "")} onValueChange={handleChange}>
          <SelectTrigger size="sm" id={`param-${name}`}>
            <SelectValue placeholder={meta.placeholder ?? "Select\u2026"} />
          </SelectTrigger>
          <SelectContent>
            {enumValues.map((enumVal) => (
              <SelectItem key={enumVal} value={enumVal}>
                {enumVal}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      ) : type === "boolean" ? (
        <Switch
          id={`param-${name}`}
          checked={Boolean(value ?? false)}
          onCheckedChange={handleChange}
        />
      ) : type === "number" ? (
        <Input
          id={`param-${name}`}
          type="number"
          value={String(value ?? "")}
          min={min}
          max={max}
          placeholder={meta.placeholder}
          onChange={(e) => {
            const num = e.target.value === "" ? undefined : Number(e.target.value);
            handleChange(num);
          }}
          className="h-8 text-sm"
        />
      ) : (
        /* string or fallback */
        <Input
          id={`param-${name}`}
          type="text"
          value={String(value ?? "")}
          placeholder={meta.placeholder}
          onChange={(e) => handleChange(e.target.value)}
          className="h-8 text-sm"
        />
      )}

      {meta.description && (
        <Text size="xs" color="muted">
          {meta.description}
        </Text>
      )}
    </div>
  );
}

export { ParameterField };
export type { ParameterFieldProps };
