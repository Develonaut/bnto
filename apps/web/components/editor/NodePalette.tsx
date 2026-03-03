"use client";

import { useCallback } from "react";
import type { NodeTypeName } from "@bnto/nodes";
import { Badge } from "@/components/ui/Badge";
import { Heading } from "@/components/ui/Heading";
import { Pressable } from "@/components/ui/Pressable";
import { Card } from "@/components/ui/Card";
import { Sheet } from "@/components/ui/Sheet";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { useNodePalette } from "@/editor/hooks/useNodePalette";
import { useEditorActions, useEditorStore } from "@/editor";
import { SLOTS } from "@/editor/adapters/bentoSlots";

/**
 * NodePalette — slide-out panel listing available node types.
 *
 * Grouped by category. Click to add a compartment at the next available
 * slot. Browser-capable nodes are addable; server-only nodes are shown
 * grayed with a "Pro" badge (definitions always visible per pricing model).
 */

interface NodePaletteProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function NodePalette({ open, onOpenChange }: NodePaletteProps) {
  const { groups } = useNodePalette();
  const { addNode } = useEditorActions();
  const nodeCount = useEditorStore((s) => s.definition.nodes?.length ?? 0);
  const isFull = nodeCount >= SLOTS.length;

  const handleAdd = useCallback(
    (typeName: NodeTypeName) => {
      addNode(typeName);
      /* Keep the palette open so users can add multiple nodes quickly. */
    },
    [addNode],
  );

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <Sheet.Content side="right" className="w-80 overflow-y-auto">
        <Sheet.Header>
          <Sheet.Title>Add Node</Sheet.Title>
          <Sheet.Description>
            Click a node type to add it to the canvas.
            {isFull && (
              <Text as="span" size="xs" color="muted" className="mt-1 block">
                Canvas is full ({SLOTS.length} nodes max).
              </Text>
            )}
          </Sheet.Description>
        </Sheet.Header>

        <Stack gap="lg" className="mt-4">
          {groups.map((group) => (
            <Stack key={group.category.name} gap="sm">
              <Heading level={4} size="xs" className="text-muted-foreground">
                {group.category.label}
              </Heading>
              <Stack gap="xs">
                {group.items.map((item) => {
                  const isServerOnly = !item.browserCapable;
                  const disabled = isFull || isServerOnly;

                  return (
                    <Pressable key={item.name} asChild spring="bouncy">
                      <button
                        type="button"
                        onClick={() => handleAdd(item.name as NodeTypeName)}
                        disabled={disabled}
                        className="w-full text-left disabled:opacity-50 disabled:cursor-not-allowed"
                      >
                        <Card
                          elevation="sm"
                          className="flex items-center gap-3 px-3 py-2.5"
                        >
                          <div className="flex flex-col flex-1 min-w-0">
                            <Text size="sm" className="font-medium truncate">
                              {item.label}
                            </Text>
                            <Text size="xs" color="muted" className="truncate">
                              {item.description}
                            </Text>
                          </div>
                          {isServerOnly ? (
                            <Badge variant="outline" className="shrink-0 text-xs">
                              Pro
                            </Badge>
                          ) : (
                            <Badge variant="secondary" className="shrink-0 text-xs">
                              Browser
                            </Badge>
                          )}
                        </Card>
                      </button>
                    </Pressable>
                  );
                })}
              </Stack>
            </Stack>
          ))}
        </Stack>
      </Sheet.Content>
    </Sheet>
  );
}

export { NodePalette };
