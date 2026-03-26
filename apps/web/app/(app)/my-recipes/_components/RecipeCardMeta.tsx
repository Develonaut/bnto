import { BlocksIcon, ClockIcon, CloudIcon, CloudOffIcon, Row, Text } from "@bnto/ui";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

interface RecipeCardMetaProps {
  nodeCount: number;
  updatedAt: number;
  syncedAt?: number | null;
}

/** Metadata row: node count, last updated, sync status. */
export function RecipeCardMeta({ nodeCount, updatedAt, syncedAt }: RecipeCardMetaProps) {
  return (
    <Row wrap className="gap-3 items-center">
      <MetaItem icon={BlocksIcon} label={nodeCount === 1 ? "1 node" : `${nodeCount} nodes`} />
      <MetaItem icon={ClockIcon} label={formatTimeAgo(updatedAt)} />
      <MetaItem
        icon={syncedAt !== null ? CloudIcon : CloudOffIcon}
        label={syncedAt !== null ? "Synced" : "Local"}
      />
    </Row>
  );
}

function MetaItem({ icon: Icon, label }: { icon: typeof BlocksIcon; label: string }) {
  return (
    <Row className="gap-1 items-center">
      <Icon className="size-3 text-muted-foreground" aria-hidden="true" />
      <Text as="span" size="xs" color="muted">
        {label}
      </Text>
    </Row>
  );
}
