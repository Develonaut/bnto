"use client";

import { LinearProgress } from "@/components/ui/LinearProgress";
import { CheckCircle2Icon, LoaderIcon } from "@/components/ui/icons";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";

/**
 * Showcase of LinearProgress primitive at different values,
 * with icons/labels, and with helper text.
 */
export function ProgressShowcase() {
  return (
    <div className="space-y-12">
      {/* Value range — bare bars at 0/25/50/75/100 */}
      <div>
        <Heading level={3} size="xs">Value Range</Heading>
        <Text size="sm" color="muted" className="mt-1 mb-4">
          Bare progress bars at 0%, 25%, 50%, 75%, 100%.
        </Text>
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
          <LinearProgress value={0} />
          <LinearProgress value={25} />
          <LinearProgress value={50} />
          <LinearProgress value={75} />
          <LinearProgress value={100} />
        </div>
      </div>

      {/* With labels & icons */}
      <div>
        <Heading level={3} size="xs">With Labels & Icons</Heading>
        <Text size="sm" color="muted" className="mt-1 mb-4">
          Real-world usage patterns: processing, complete, file counter, initializing.
        </Text>
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
          <LinearProgress
            value={42}
            icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
            label="Compressing..."
          />
          <LinearProgress
            value={100}
            icon={<CheckCircle2Icon className="size-4 shrink-0 text-primary" />}
            label="Complete"
            valueLabel="2.4 MB saved"
          />
          <LinearProgress
            value={67}
            label="Processing file 2 of 3"
          />
          <LinearProgress
            value={0}
            icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
            label="Initializing..."
            valueLabel=""
          />
        </div>
      </div>

      {/* With helper text */}
      <div>
        <Heading level={3} size="xs">With Helper Text</Heading>
        <Text size="sm" color="muted" className="mt-1 mb-4">
          Helper text below the bar provides additional context.
        </Text>
        <div className="grid grid-cols-1 gap-6 sm:grid-cols-2">
          <LinearProgress
            value={35}
            icon={<LoaderIcon className="size-4 shrink-0 text-primary motion-safe:animate-spin" />}
            label="Uploading..."
            helperText="3 of 8 files uploaded (12.4 MB remaining)"
          />
          <LinearProgress
            value={100}
            icon={<CheckCircle2Icon className="size-4 shrink-0 text-primary" />}
            label="Complete"
            valueLabel="100%"
            helperText="All files processed successfully"
          />
        </div>
      </div>
    </div>
  );
}
