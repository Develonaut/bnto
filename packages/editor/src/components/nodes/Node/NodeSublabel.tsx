import { Text } from "@bnto/ui";

/** NodeSublabel — secondary description text below the label. */

function NodeSublabel({ children }: { children?: string }) {
  if (!children) return null;
  return (
    <Text size="xs" color="muted">
      {children}
    </Text>
  );
}

export { NodeSublabel };
