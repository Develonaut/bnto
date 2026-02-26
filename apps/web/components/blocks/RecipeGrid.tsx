"use client";

import Link from "next/link";

import {
  ArrowRightLeftIcon,
  Columns3Icon,
  type LucideIcon,
  Minimize2Icon,
  PenLineIcon,
  ScalingIcon,
  SparklesIcon,
} from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { BentoGrid, useBentoItem } from "@/components/ui/BentoGrid";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { BNTO_REGISTRY, type BntoEntry } from "@/lib/bntoRegistry";

/* ── Icon map ────────────────────────────────────────────────── */

const BNTO_ICONS: Record<string, LucideIcon> = {
  "compress-images": Minimize2Icon,
  "resize-images": ScalingIcon,
  "convert-image-format": ArrowRightLeftIcon,
  "rename-files": PenLineIcon,
  "clean-csv": SparklesIcon,
  "rename-csv-columns": Columns3Icon,
};

/* ── Recipe cell ─────────────────────────────────────────────── */

function RecipeCell({ entry }: { entry: BntoEntry }) {
  const { featured } = useBentoItem();
  const Icon = BNTO_ICONS[entry.slug] ?? SparklesIcon;

  return (
    <Link href={`/${entry.slug}`} className="group h-full">
      <Button
        variant="outline"
        asChild
        className="flex h-full items-stretch justify-start whitespace-normal text-left"
      >
        <Card className="flex flex-col justify-between p-5">
          <div className="flex items-start justify-between">
            <div className="bg-primary/10 text-primary flex size-10 items-center justify-center rounded-lg">
              <Icon className="size-5" />
            </div>
            <Text
              as="span"
              size="xs"
              mono
              color="muted"
              className="uppercase tracking-wider"
            >
              {entry.features[0]}
            </Text>
          </div>

          <div className="mt-auto flex flex-col gap-1.5 pt-4">
            <Heading level={3} size="xs" className="text-left">
              {entry.h1.replace(/ Online Free$/, "")}
            </Heading>
            {featured && (
              <Text
                size="sm"
                color="muted"
                leading="snug"
                className="text-left"
              >
                {entry.description}
              </Text>
            )}
            <div className="flex flex-wrap gap-1.5 pt-1">
              {entry.features.slice(0, featured ? 5 : 3).map((f) => (
                <span
                  key={f}
                  className="bg-muted text-muted-foreground rounded-full px-2 py-0.5 text-xs font-medium"
                >
                  {f}
                </span>
              ))}
            </div>
          </div>
        </Card>
      </Button>
    </Link>
  );
}

/* ── Recipe grid ─────────────────────────────────────────────── */

export function RecipeGrid() {
  return (
    <Animate.Stagger className="flex flex-col gap-3">
      <BentoGrid cols={2} uniform>
        {BNTO_REGISTRY.map((entry, i) => (
          <Animate.ScaleIn
            key={entry.slug}
            index={i}
            from={0.85}
            easing="spring-bouncy"
            className="h-full"
          >
            <RecipeCell entry={entry} />
          </Animate.ScaleIn>
        ))}
      </BentoGrid>
      <Text size="xs" color="muted" className="text-center">
        Pick a tool to get started. No signup needed.
      </Text>
    </Animate.Stagger>
  );
}
