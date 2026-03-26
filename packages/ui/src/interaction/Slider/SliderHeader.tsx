import type { ReactNode } from "react";

import { Label } from "../../typography/Label";
import { Text } from "../../typography/Text";

export function SliderHeader({
  label,
  annotation,
}: {
  label: ReactNode;
  annotation: string | null;
}) {
  return (
    <div className="flex items-center justify-between gap-4">
      {typeof label === "string" ? <Label>{label}</Label> : label}
      {annotation && (
        <Text size="xs" mono color="muted">
          {annotation}
        </Text>
      )}
    </div>
  );
}
