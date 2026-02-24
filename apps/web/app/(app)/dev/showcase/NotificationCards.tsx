import { Card } from "@/components/ui/Card";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

export function NotificationCards() {
  return (
    <Stack className="gap-3">
      <Card className="flex items-center gap-4 p-4">
        <div className="flex size-10 items-center justify-center rounded-full bg-primary text-primary-foreground font-display font-bold">
          B
        </div>
        <div className="flex-1">
          <Text size="sm" weight="semibold" as="span">
            Workflow completed
          </Text>
          <Text size="xs" color="muted">
            compress-images finished in 2.4s
          </Text>
        </div>
        <Text size="xs" color="muted" as="span">2m ago</Text>
      </Card>

      <Card className="flex items-center gap-4 p-4">
        <div className="flex size-10 items-center justify-center rounded-full bg-accent text-accent-foreground font-display font-bold">
          3
        </div>
        <div className="flex-1">
          <Text size="sm" weight="semibold" as="span">
            3 files processed
          </Text>
          <Text size="xs" color="muted">
            resize-images batch complete
          </Text>
        </div>
        <Text size="xs" color="muted" as="span">5m ago</Text>
      </Card>

      <Card className="flex items-center gap-4 p-4">
        <div className="flex size-10 items-center justify-center rounded-full bg-destructive text-destructive-foreground font-display font-bold">
          !
        </div>
        <div className="flex-1">
          <Text size="sm" weight="semibold" as="span">
            Execution failed
          </Text>
          <Text size="xs" color="muted">
            call-api returned 503 &mdash; retry available
          </Text>
        </div>
        <Text size="xs" color="muted" as="span">12m ago</Text>
      </Card>
    </Stack>
  );
}
