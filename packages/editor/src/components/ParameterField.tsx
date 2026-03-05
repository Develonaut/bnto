"use client";

import { useCallback } from "react";
import type { ParameterSchema } from "@bnto/nodes";
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
 * ParameterField — auto-generated form field from ParameterSchema.
 *
 * Renders the appropriate control based on param.type:
 *   string → text input
 *   number → number input with min/max
 *   boolean → switch toggle
 *   enum → select dropdown
 */

interface ParameterFieldProps {
  param: ParameterSchema;
  value: unknown;
  onChange: (name: string, value: unknown) => void;
}

function ParameterField({ param, value, onChange }: ParameterFieldProps) {
  const handleChange = useCallback(
    (newValue: unknown) => onChange(param.name, newValue),
    [param.name, onChange],
  );

  return (
    <div className="flex flex-col gap-1.5">
      <Label htmlFor={`param-${param.name}`} className="text-xs font-medium">
        {param.label}
        {param.required && <span className="ml-0.5 text-destructive">*</span>}
      </Label>

      {param.type === "enum" && param.enumValues ? (
        <Select value={String(value ?? param.default ?? "")} onValueChange={handleChange}>
          <SelectTrigger size="sm" id={`param-${param.name}`}>
            <SelectValue placeholder={param.placeholder ?? "Select\u2026"} />
          </SelectTrigger>
          <SelectContent>
            {param.enumValues.map((enumVal) => (
              <SelectItem key={enumVal} value={enumVal}>
                {enumVal}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      ) : param.type === "boolean" ? (
        <Switch
          id={`param-${param.name}`}
          checked={Boolean(value ?? param.default ?? false)}
          onCheckedChange={handleChange}
        />
      ) : param.type === "number" ? (
        <Input
          id={`param-${param.name}`}
          type="number"
          value={String(value ?? param.default ?? "")}
          min={param.min}
          max={param.max}
          placeholder={param.placeholder}
          onChange={(e) => {
            const num = e.target.value === "" ? undefined : Number(e.target.value);
            handleChange(num);
          }}
          className="h-8 text-sm"
        />
      ) : (
        /* string or fallback */
        <Input
          id={`param-${param.name}`}
          type="text"
          value={String(value ?? param.default ?? "")}
          placeholder={param.placeholder}
          onChange={(e) => handleChange(e.target.value)}
          className="h-8 text-sm"
        />
      )}

      {param.description && (
        <Text size="xs" color="muted">
          {param.description}
        </Text>
      )}
    </div>
  );
}

export { ParameterField };
