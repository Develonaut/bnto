import { CloudIcon, CloudOffIcon, IconBadge } from "@bnto/ui";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

/** Sync icon badge — replaces the recipe icon to show cloud sync state. */
export function SyncStatus({ syncedAt }: { syncedAt?: number | null }) {
  const isSynced = syncedAt !== undefined && syncedAt !== null && syncedAt > 0;

  return (
    <IconBadge
      variant={isSynced ? "primary" : "muted"}
      size="lg"
      shape="square"
      title={isSynced ? `Synced ${formatTimeAgo(syncedAt)}` : "Local only — sign in to sync"}
    >
      {isSynced ? <CloudIcon className="size-5" /> : <CloudOffIcon className="size-5" />}
    </IconBadge>
  );
}
