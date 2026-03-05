"use client";

import {
  Button,
  Card,
  DownloadIcon,
  GridItem,
  PlayIcon,
  RadialSlider,
  Row,
  Switch,
  Text,
  TrashIcon,
} from "@bnto/ui";

export function QualitySliderPanel({
  value,
  onChange,
}: {
  value: number;
  onChange: (v: number) => void;
}) {
  return (
    <GridItem colSpan={3} rowSpan={2} colStart={4} rowStart={1}>
      <Card className="flex h-full items-center justify-center p-4" elevation="sm">
        <RadialSlider
          min={1}
          max={100}
          value={value}
          onChange={onChange}
          startAngle={135}
          endAngle={405}
          size={120}
          aria-label="Quality"
        >
          <div className="flex flex-col items-center">
            <span className="text-2xl font-bold tabular-nums">{value}</span>
            <span className="text-xs text-muted-foreground">Quality</span>
          </div>
        </RadialSlider>
      </Card>
    </GridItem>
  );
}

export function AutoDownloadTile({ defaultChecked = false }: { defaultChecked?: boolean }) {
  return (
    <GridItem colSpan={1} rowSpan={1} colStart={7} rowStart={2}>
      <Card className="flex h-full flex-col items-center justify-center gap-2 p-4" elevation="sm">
        <Switch defaultChecked={defaultChecked} />
        <Text size="xs" color="muted" className="text-center leading-tight">
          Auto
          <br />
          download
        </Text>
      </Card>
    </GridItem>
  );
}

export function ActionBar({
  trashDisabled = false,
  runDisabled = false,
  downloadDisabled = false,
}: {
  trashDisabled?: boolean;
  runDisabled?: boolean;
  downloadDisabled?: boolean;
}) {
  return (
    <Row gap="sm" justify="end" align="end" className="h-full">
      <Button variant="outline" size="icon" elevation="md" disabled={trashDisabled}>
        <TrashIcon className="size-4" />
      </Button>
      <Button variant="primary" disabled={runDisabled}>
        <PlayIcon className="size-4" />
        Run
      </Button>
      <Button variant="outline" size="icon" elevation="md" disabled={downloadDisabled}>
        <DownloadIcon className="size-4" />
      </Button>
    </Row>
  );
}
