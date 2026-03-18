import { CloudIcon, CloudOffIcon } from "@bnto/ui";

/** Sync status icon — cloud if synced, cloud-off if local-only. */
export function SyncIcon({ syncedAt }: { syncedAt?: number | null }) {
  if (syncedAt === undefined || (syncedAt !== null && syncedAt > 0)) {
    return <CloudIcon className="size-3 text-muted-foreground" />;
  }
  return <CloudOffIcon className="size-3 text-muted-foreground" />;
}
