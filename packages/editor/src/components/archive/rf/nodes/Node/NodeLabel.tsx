import { Text } from "@bnto/ui";

/** NodeLabel — the node's primary label text. */

function NodeLabel({ children, onSurface }: { children: string; onSurface?: boolean }) {
  return (
    <Text
      size="sm"
      color={onSurface ? "inherit" : "default"}
      className="font-display font-semibold leading-tight text-center"
    >
      {children}
    </Text>
  );
}

export { NodeLabel };
