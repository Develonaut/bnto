"use client";

import { useState } from "react";
import { Card, UploadIcon } from "@bnto/ui";
import { PhaseShell } from "./PhaseShell";
import { AcceptsCard } from "./MockCards";
import { ActionBar, AutoDownloadTile, QualitySliderPanel } from "./SharedPanels";
import { StatsCard } from "./StatsCard";

export function Phase1() {
  const [quality, setQuality] = useState(80);

  return (
    <PhaseShell
      phase={1}
      controlPanel={
        <>
          <QualitySliderPanel value={quality} onChange={setQuality} />
          <AutoDownloadTile />
        </>
      }
      actionBar={<ActionBar trashDisabled runDisabled downloadDisabled />}
      statsCard={
        <StatsCard
          title="Waiting for files"
          stats={[
            { value: "\u2014", label: "files" },
            { value: "\u2014", label: "MB in" },
            { value: "\u2014", label: "MB est." },
          ]}
          muted
        />
      }
      leftBottomCard={<AcceptsCard />}
      mainContent={
        <Card
          className="flex h-full flex-col items-center justify-center gap-3 border-2 border-dashed border-border p-8"
          elevation="sm"
        >
          <div className="rounded-full bg-muted p-3 text-muted-foreground">
            <UploadIcon className="size-6" />
          </div>
          <div className="text-center">
            <p className="text-sm font-medium text-foreground">
              Drag &amp; drop files here
            </p>
            <p className="mt-1 text-xs text-muted-foreground">
              or click to browse &middot; accepts JPEG, PNG, WebP
            </p>
          </div>
        </Card>
      }
    />
  );
}
