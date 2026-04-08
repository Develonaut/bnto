"use client";

import { Surface } from "@bnto/ui";

import { useStepInView } from "@/app/(app)/_components/useStepInView";

import { Terminal } from "./Terminal";

const DEMO_LINES = [
  { text: "$ cargo install bnto", typing: true, speed: 35, delay: 400 },
  { text: "  Installed bnto v0.5.0", className: "text-muted-foreground", delay: 600 },
  { text: "", delay: 200 },
  { text: "$ bnto run compress-images photos/*.jpg", typing: true, speed: 30, delay: 500 },
  {
    text: "  ✓ photo1.jpg  2.4 MB → 890 KB  (-63%)",
    className: "text-secondary-foreground",
    delay: 400,
  },
  {
    text: "  ✓ photo2.jpg  1.8 MB → 640 KB  (-64%)",
    className: "text-secondary-foreground",
    delay: 300,
  },
  {
    text: "  ✓ photo3.jpg  3.1 MB → 1.1 MB  (-65%)",
    className: "text-secondary-foreground",
    delay: 300,
  },
  { text: "", delay: 100 },
  { text: "  Done in 127ms", className: "text-muted-foreground", delay: 200 },
];

/** Hero terminal — animated CLI demo showing install + run flow. */
export function HeroTerminal() {
  const [inView, ref] = useStepInView(0.3);

  return (
    <Surface ref={ref} dormant={!inView} elevation="lg" rounded="xl">
      <Terminal lines={DEMO_LINES} />
    </Surface>
  );
}
