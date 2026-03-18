import { CloudIcon, CloudOffIcon, Row, Text } from "@bnto/ui";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/** Sync status — icon + descriptive text showing cloud sync state. */
export function SyncStatus({ syncedAt }: { syncedAt?: number | null }) {
  const isSynced = syncedAt !== undefined && syncedAt !== null && syncedAt > 0;

  return (
    <Row className="gap-1 items-center">
      {isSynced ? (
        <>
          <CloudIcon className="size-3 text-secondary-foreground" />
          <Text as="span" size="xs" color="muted">
            Synced {formatTimeAgo(syncedAt)}
          </Text>
        </>
      ) : (
        <>
          <CloudOffIcon className="size-3 text-muted-foreground" />
          <Text as="span" size="xs" color="muted">
            Local only
          </Text>
        </>
      )}
    </Row>
  );
}
