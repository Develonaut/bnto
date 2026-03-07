import { Text } from "@bnto/ui";

/** NodeLabel — the node's primary label text. */

function NodeLabel({ children }: { children: string }) {
  return (
    <Text size="sm" className="font-display font-semibold leading-tight text-center">
      {children}
    </Text>
  );
}

export { NodeLabel };
