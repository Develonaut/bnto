"use client";

import { useState, useCallback, type ComponentType } from "react";
import {
  Button,
  Card,
  Text,
  ScaleIn,
  Stagger,
  PresenceList,
  Pressable,
  PlusIcon,
  MinusIcon,
  XIcon,
  ImageIcon,
  FileTextIcon,
  ArrowRightLeftIcon,
  SparklesIcon,
  ScalingIcon,
  PenLineIcon,
  Columns3Icon,
  Minimize2Icon,
  RotateCcwIcon,
} from "@bnto/ui";

/* ── Mock node data ───────────────────────────────────────────── */

interface MockNode {
  id: string;
  label: string;
  sublabel: string;
  icon: ComponentType<{ className?: string }>;
  isIoNode: boolean;
}

const NODE_POOL: Omit<MockNode, "id">[] = [
  { label: "Compress", sublabel: "image", icon: Minimize2Icon, isIoNode: false },
  { label: "Resize", sublabel: "image", icon: ScalingIcon, isIoNode: false },
  { label: "Convert", sublabel: "transform", icon: ArrowRightLeftIcon, isIoNode: false },
  { label: "Clean", sublabel: "data", icon: SparklesIcon, isIoNode: false },
  { label: "Rename", sublabel: "file", icon: PenLineIcon, isIoNode: false },
  { label: "Columns", sublabel: "data", icon: Columns3Icon, isIoNode: false },
];

const IO_NODES: MockNode[] = [
  { id: "input", label: "Input", sublabel: "files", icon: ImageIcon, isIoNode: true },
  { id: "output", label: "Output", sublabel: "results", icon: FileTextIcon, isIoNode: true },
];

let nodeCounter = 0;

/* ── Node size constants ──────────────────────────────────────── */

const IO_SIZE = 120;
const COMPARTMENT_SIZE = 160;

/* ── Shared node internals ───────────────────────────────────── */

interface NodeCardProps {
  node: MockNode;
  size: number;
  selected: boolean;
  onSelect: () => void;
  onDelete?: () => void;
}

function NodeCard({ node, size, selected, onSelect, onDelete }: NodeCardProps) {
  const Icon = node.icon;

  return (
    <div className="group relative" onClick={onSelect}>
      <Pressable asChild spring="bounciest" toggle active={selected}>
        <Card
          elevation={selected ? "lg" : "sm"}
          className="flex flex-col items-center justify-center gap-1 rounded-xl"
          style={{ width: size, height: size }}
        >
          <Icon className="size-8 text-muted-foreground" />
          <Text size="sm" className="font-display font-semibold leading-tight text-center">
            {node.label}
          </Text>
          <Text size="xs" color="muted" className="capitalize">
            {node.sublabel}
          </Text>
        </Card>
      </Pressable>

      {onDelete && (
        <button
          type="button"
          onClick={(e) => {
            e.stopPropagation();
            onDelete();
          }}
          className="absolute -right-1.5 -top-1.5 flex size-6 items-center justify-center rounded-full
            bg-destructive text-destructive-foreground opacity-0 shadow-sm transition-opacity
            group-hover:opacity-100 group-focus-within:opacity-100"
          aria-label="Delete node"
        >
          <XIcon className="size-3" />
        </button>
      )}
    </div>
  );
}

/* ── Typed node components ───────────────────────────────────── */

type SelectableProps = { node: MockNode; selected: boolean; onSelect: () => void };

function IONode(props: SelectableProps) {
  return <NodeCard {...props} size={IO_SIZE} />;
}

function CompartmentNode(props: SelectableProps & { onDelete?: () => void }) {
  return <NodeCard {...props} size={COMPARTMENT_SIZE} />;
}

/* ── Main showcase ────────────────────────────────────────────── */

function createInitialNodes(): MockNode[] {
  nodeCounter = 0;
  return [IO_NODES[0], { ...NODE_POOL[0], id: `node-${++nodeCounter}` }, IO_NODES[1]];
}

export function EditorShowcase() {
  const [nodes, setNodes] = useState<MockNode[]>(createInitialNodes);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const processingNodes = nodes.filter((n) => !n.isIoNode);
  const processingKeys = processingNodes.map((n) => n.id);
  const nodeMap = new Map(processingNodes.map((n) => [n.id, n]));

  const addNode = useCallback(() => {
    const poolIndex = nodeCounter % NODE_POOL.length;
    const template = NODE_POOL[poolIndex];
    const newNode: MockNode = { ...template, id: `node-${++nodeCounter}` };
    setNodes((prev) => {
      const copy = [...prev];
      copy.splice(copy.length - 1, 0, newNode);
      return copy;
    });
  }, []);

  const removeNode = useCallback((id: string) => {
    setNodes((prev) => prev.filter((n) => n.id !== id));
    setSelectedId((s) => (s === id ? null : s));
  }, []);

  const removeLast = useCallback(() => {
    const lastProcessing = [...nodes].reverse().find((n) => !n.isIoNode);
    if (!lastProcessing) return;
    removeNode(lastProcessing.id);
  }, [nodes, removeNode]);

  const reset = useCallback(() => {
    setNodes(createInitialNodes());
    setSelectedId(null);
  }, []);

  return (
    <div className="space-y-8">
      {/* ── Enter + Exit ───────────────────────────────── */}
      <div>
        <Text size="sm" weight="medium" color="muted" className="mb-4">
          Add and remove nodes to see spring enter/exit/layout animations. I/O nodes (Input, Output)
          are always present.
        </Text>

        <div className="flex flex-wrap items-end gap-4">
          <Button
            variant="outline"
            elevation={false}
            onClick={addNode}
            disabled={processingNodes.length >= 6}
          >
            <PlusIcon className="size-3.5" />
            Add Node
          </Button>

          <Button
            variant="outline"
            elevation={false}
            onClick={removeLast}
            disabled={processingNodes.length === 0}
          >
            <MinusIcon className="size-3.5" />
            Remove Last
          </Button>

          <Button variant="outline" elevation={false} onClick={reset}>
            <RotateCcwIcon className="size-3.5" />
            Reset
          </Button>
        </div>

        <div className="mt-6 flex flex-wrap items-center gap-4">
          {/* Input — always present */}
          <IONode
            key="input"
            node={IO_NODES[0]}
            selected={selectedId === "input"}
            onSelect={() => setSelectedId("input")}
          />

          {/* Processing nodes — animated enter/exit/layout */}
          <PresenceList keys={processingKeys}>
            {(key) => {
              const node = nodeMap.get(key);
              if (!node) return null;
              return (
                <CompartmentNode
                  node={node}
                  selected={selectedId === key}
                  onSelect={() => setSelectedId(key)}
                  onDelete={() => removeNode(key)}
                />
              );
            }}
          </PresenceList>

          {/* Output — always present */}
          <IONode
            key="output"
            node={IO_NODES[1]}
            selected={selectedId === "output"}
            onSelect={() => setSelectedId("output")}
          />
        </div>
      </div>

      {/* ── Stagger Enter ──────────────────────────────── */}
      <StaggerEnterDemo />
    </div>
  );
}

/* ── Stagger demo ─────────────────────────────────────────────── */

function StaggerEnterDemo() {
  const [key, setKey] = useState(0);

  return (
    <div>
      <div className="mb-3 flex items-center justify-between">
        <Text size="sm" weight="medium" color="muted">
          Stagger entrance. All compartments pop onto the grid like buildings materializing
        </Text>
        <Button variant="outline" elevation={false} onClick={() => setKey((k) => k + 1)}>
          <RotateCcwIcon className="size-3.5" />
          Replay
        </Button>
      </div>

      <Stagger key={key} className="flex flex-wrap items-center gap-4">
        {[
          IO_NODES[0],
          ...NODE_POOL.slice(0, 3).map((n, i) => ({ ...n, id: `stagger-${i}` })),
          IO_NODES[1],
        ].map((node, i) => (
          <ScaleIn key={node.id ?? `s-${i}`} index={i} from={0.7} easing="spring-bouncier">
            <Card
              elevation="sm"
              className="flex flex-col items-center justify-center gap-1 rounded-xl"
              style={{
                width: node.isIoNode ? IO_SIZE : COMPARTMENT_SIZE,
                height: node.isIoNode ? IO_SIZE : COMPARTMENT_SIZE,
              }}
            >
              <node.icon className="size-8 text-muted-foreground" />
              <Text size="sm" className="font-display font-semibold leading-tight text-center">
                {node.label}
              </Text>
            </Card>
          </ScaleIn>
        ))}
      </Stagger>
    </div>
  );
}
