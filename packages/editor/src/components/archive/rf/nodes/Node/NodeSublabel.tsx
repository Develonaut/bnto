import { Text } from "@bnto/ui";

/** NodeSublabel — secondary label below the node's primary label. */

function NodeSublabel({ children, onSurface }: { children: string; onSurface?: boolean }) {
  return (
    <Text
      size="xs"
      color={onSurface ? "inherit" : "muted"}
      className="leading-tight text-center"
    >
      {children}
    </Text>
  );
}

export { NodeSublabel };
