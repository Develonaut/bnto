"use client";

import { useState } from "react";
import { PhaseShell } from "./PhaseShell";
import { AcceptsCard, MockFileCard } from "./MockCards";
import { ActionBar, AutoDownloadTile, QualitySliderPanel } from "./SharedPanels";
import { StatsCard } from "./StatsCard";

export function Phase2() {
  const [quality, setQuality] = useState(80);

  return (
    <PhaseShell
      phase={2}
      controlPanel={
        <>
          <QualitySliderPanel value={quality} onChange={setQuality} />
          <AutoDownloadTile />
        </>
      }
      actionBar={<ActionBar downloadDisabled />}
      statsCard={
        <StatsCard
          title="Ready to compress"
          stats={[
            { value: "2", label: "files" },
            { value: "4.2", label: "MB in" },
            { value: "~1.1", label: "MB est." },
          ]}
        />
      }
      leftBottomCard={<AcceptsCard />}
      mainContent={
        <div className="flex h-full flex-col gap-2">
          <MockFileCard name="hero-banner.jpg" size="2.4 MB" />
          <MockFileCard name="product-shot.png" size="1.8 MB" />
        </div>
      }
    />
  );
}
