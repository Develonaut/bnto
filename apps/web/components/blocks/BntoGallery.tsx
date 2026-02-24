import Link from "next/link";

import type { LucideIcon } from "lucide-react";
import {
  ArrowRightLeft,
  Columns3,
  Minimize2,
  PenLine,
  Scaling,
  Sparkles,
} from "lucide-react";

import { DashedLine } from "@/components/DashedLine";
import { Card } from "@/components/ui/Card";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { BNTO_REGISTRY, type BntoEntry } from "@/lib/bntoRegistry";

const BNTO_ICONS: Record<string, LucideIcon> = {
  "compress-images": Minimize2,
  "resize-images": Scaling,
  "convert-image-format": ArrowRightLeft,
  "rename-files": PenLine,
  "clean-csv": Sparkles,
  "rename-csv-columns": Columns3,
};

function BntoCard({ entry }: { entry: BntoEntry }) {
  const Icon = BNTO_ICONS[entry.slug] ?? Sparkles;

  return (
    <Link href={`/${entry.slug}`} className="group">
      <Card className="h-full motion-safe:transition-shadow motion-safe:duration-normal hover:shadow-md">
        <Card.Content className="flex flex-col gap-3 p-5">
          <div className="bg-primary/10 text-primary-foreground flex size-10 items-center justify-center rounded-lg">
            <Icon className="size-5" />
          </div>
          <Heading level={3} size="xs">
            {entry.h1.replace(/ Online Free$/, "")}
          </Heading>
          <Text size="sm" color="muted" leading="relaxed">
            {entry.description}
          </Text>
          <div className="mt-auto flex flex-wrap gap-1.5 pt-1">
            {entry.features.slice(0, 4).map((feature) => (
              <span
                key={feature}
                className="bg-muted text-muted-foreground rounded-full px-2.5 py-0.5 text-xs font-medium"
              >
                {feature}
              </span>
            ))}
          </div>
        </Card.Content>
      </Card>
    </Link>
  );
}

export const BntoGallery = () => {
  return (
    <section id="tools" className="pb-28 lg:pb-32">
      <Container>
        {/* Dashed divider with label */}
        <div className="relative flex items-center justify-center">
          <DashedLine className="text-muted-foreground" />
          <span className="bg-muted text-muted-foreground absolute px-3 font-mono text-sm font-medium tracking-wide max-md:hidden">
            FREE. NO SIGNUP.
          </span>
        </div>

        {/* Heading */}
        <div className="mx-auto mt-10 grid max-w-4xl items-center gap-3 md:gap-0 lg:mt-24 lg:grid-cols-2">
          <Heading level={2} className="text-balance">
            Pick a tool. Drop your files.
          </Heading>
          <Text color="muted" leading="snug">
            Compress images, clean CSVs, rename files — all in your browser.
            No upload limits, no account required.
          </Text>
        </div>

        {/* Card grid */}
        <div className="mt-8 grid gap-4 sm:grid-cols-2 lg:mt-16 lg:grid-cols-3">
          {BNTO_REGISTRY.map((entry) => (
            <BntoCard key={entry.slug} entry={entry} />
          ))}
        </div>
      </Container>
    </section>
  );
};
