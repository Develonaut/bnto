"use client";

import { useState } from "react";

import {
  CheckIcon,
  ClockIcon,
  DownloadIcon,
  ImageIcon,
  List,
  ListItem,
  ListItemActions,
  ListItemContent,
  ListSeparator,
  ListSubheader,
  Row,
  ShuffleIcon,
  Stack,
  StarIcon,
  Surface,
  TableIcon,
  Text,
  TrashIcon,
  ZapIcon,
} from "@bnto/ui";

export function ListShowcase() {
  const [selected, setSelected] = useState("compress");

  return (
    <Stack className="gap-8">
      {/* ── Notifications ─────────────────────────────────── */}
      <Stack className="gap-1.5">
        <Text size="xs" color="muted" className="px-1 font-mono uppercase tracking-wider">
          Notifications
        </Text>
        <Surface elevation="sm" rounded="xl" className="p-1.5">
          <List>
            <ListItem>
              <div className="flex size-10 items-center justify-center rounded-full bg-primary font-display font-bold text-primary-foreground">
                B
              </div>
              <ListItemContent>
                <Text size="sm" weight="semibold" as="span">
                  Recipe completed
                </Text>
                <Text size="xs" color="muted">
                  compress-images finished in 2.4s
                </Text>
              </ListItemContent>
              <Text size="xs" color="muted" as="span">
                2m ago
              </Text>
            </ListItem>

            <ListItem>
              <div className="flex size-10 items-center justify-center rounded-full bg-accent font-display font-bold text-accent-foreground">
                3
              </div>
              <ListItemContent>
                <Text size="sm" weight="semibold" as="span">
                  3 files processed
                </Text>
                <Text size="xs" color="muted">
                  resize-images batch complete
                </Text>
              </ListItemContent>
              <Text size="xs" color="muted" as="span">
                5m ago
              </Text>
            </ListItem>

            <ListItem>
              <div className="flex size-10 items-center justify-center rounded-full bg-destructive font-display font-bold text-destructive-foreground">
                !
              </div>
              <ListItemContent>
                <Text size="sm" weight="semibold" as="span">
                  Execution failed
                </Text>
                <Text size="xs" color="muted">
                  call-api returned 503, retry available
                </Text>
              </ListItemContent>
              <Text size="xs" color="muted" as="span">
                12m ago
              </Text>
            </ListItem>
          </List>
        </Surface>
      </Stack>

      {/* ── Icons + Subheaders + Separator ─────────────────── */}
      <Stack className="gap-1.5">
        <Text size="xs" color="muted" className="px-1 font-mono uppercase tracking-wider">
          Grouped with subheaders
        </Text>
        <Surface elevation="sm" rounded="xl" className="p-1.5">
          <List>
            <ListSubheader>Recipes</ListSubheader>
            <ListItem>
              <ImageIcon />
              <ListItemContent>Compress Images</ListItemContent>
              <ListItemActions>
                <ZapIcon className="size-3.5 text-accent-foreground" />
              </ListItemActions>
            </ListItem>
            <ListItem>
              <TableIcon />
              <ListItemContent>Clean CSV</ListItemContent>
            </ListItem>
            <ListItem>
              <ShuffleIcon />
              <ListItemContent>Rename Files</ListItemContent>
            </ListItem>

            <ListSeparator />

            <ListSubheader>Recent</ListSubheader>
            <ListItem>
              <ClockIcon />
              <ListItemContent>Resize batch — 14 files</ListItemContent>
              <Text size="xs" color="muted" as="span">
                3h ago
              </Text>
            </ListItem>
            <ListItem>
              <ClockIcon />
              <ListItemContent>CSV cleanup — contacts.csv</ListItemContent>
              <Text size="xs" color="muted" as="span">
                yesterday
              </Text>
            </ListItem>
          </List>
        </Surface>
      </Stack>

      {/* ── Selectable ────────────────────────────────────── */}
      <Stack className="gap-1.5">
        <Text size="xs" color="muted" className="px-1 font-mono uppercase tracking-wider">
          Selectable with border
        </Text>
        <Surface elevation="sm" rounded="xl" className="p-1.5">
          <List>
            {[
              { id: "compress", icon: ImageIcon, label: "Compress Images" },
              { id: "csv", icon: TableIcon, label: "Clean CSV" },
              { id: "rename", icon: ShuffleIcon, label: "Rename Files" },
            ].map(({ id, icon: Icon, label }) => (
              <ListItem
                key={id}
                selectable
                selected={selected === id}
                onClick={() => setSelected(id)}
              >
                <Icon />
                <ListItemContent>{label}</ListItemContent>
                {selected === id && <CheckIcon className="size-4 text-primary" />}
              </ListItem>
            ))}
          </List>
        </Surface>
      </Stack>

      {/* ── Actions ───────────────────────────────────────── */}
      <Stack className="gap-1.5">
        <Text size="xs" color="muted" className="px-1 font-mono uppercase tracking-wider">
          Action rows
        </Text>
        <Surface elevation="sm" rounded="xl" className="p-1.5">
          <List>
            <ListItem selectable>
              <StarIcon />
              <ListItemContent>Add to favorites</ListItemContent>
            </ListItem>
            <ListItem selectable>
              <DownloadIcon />
              <ListItemContent>Download recipe</ListItemContent>
              <ListItemActions>
                <Row className="gap-1">
                  <Text size="xs" color="muted" as="span" className="font-mono">
                    .bnto.json
                  </Text>
                </Row>
              </ListItemActions>
            </ListItem>
            <ListSeparator />
            <ListItem selectable className="text-destructive hover:bg-destructive/10">
              <TrashIcon />
              <ListItemContent>Delete recipe</ListItemContent>
            </ListItem>
          </List>
        </Surface>
      </Stack>
    </Stack>
  );
}
