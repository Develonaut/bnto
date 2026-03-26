import type { ReactNode } from "react";
import type { LucideIcon } from "../icons";
import { Row } from "../layout/Row";
import { Text } from "./Text";

interface MetaItemProps {
  icon: LucideIcon;
  children: ReactNode;
}

/** Icon + text pair for displaying metadata (node count, last updated, etc). */
function MetaItem({ icon: Icon, children }: MetaItemProps) {
  return (
    <Row className="gap-1 items-center">
      <Icon className="size-3 text-muted-foreground" aria-hidden="true" />
      <Text as="span" size="xs" color="muted">
        {children}
      </Text>
    </Row>
  );
}

export { MetaItem };
