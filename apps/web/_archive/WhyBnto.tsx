import {
  GlobeIcon,
  CodeIcon,
  UserXIcon,
  ZapIcon,
  MonitorIcon,
} from "@bnto/ui";
import type { LucideIcon } from "@bnto/ui";
import { SectionLabel } from "@bnto/ui/section-label";
import { DashedLine } from "@bnto/ui/dashed-line";

export function WhyBnto() {
  return (
    <section className="container flex flex-col gap-12 py-12 lg:py-16">
      <SectionLabel>Why bnto</SectionLabel>

      <div className="grid gap-4 lg:grid-cols-2 lg:gap-8">
        <h2 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
          Built for simplicity
        </h2>
        <p className="text-muted-foreground text-lg leading-relaxed text-balance">
          No platforms to learn, no accounts to create, no files leaving your
          machine. Just open a tool and get things done.
        </p>
      </div>

      {/* Top row: 2 items */}
      <div className="grid gap-6 lg:grid-cols-2">
        <div className="flex gap-6">
          <FeatureCard
            icon={GlobeIcon}
            title="Browser-based processing"
            description="Files never leave your device. Every tool runs locally in your browser — no server uploads, complete privacy."
          />
          <DashedLine orientation="vertical" className="hidden lg:block" />
        </div>
        <FeatureCard
          icon={CodeIcon}
          title="Open source, MIT licensed"
          description="Inspect the code, contribute features, self-host. No black boxes, no vendor lock-in."
        />
      </div>

      <DashedLine />

      {/* Bottom row: 3 items */}
      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
        <FeatureCard
          icon={UserXIcon}
          title="No account needed"
          description="Start using tools immediately. No sign-up walls, no email verification."
        />
        <FeatureCard
          icon={ZapIcon}
          title="Unlimited files"
          description="No file count or size limits. Process as many files as your browser can handle."
        />
        <FeatureCard
          icon={MonitorIcon}
          title="Desktop app coming soon"
          description="A native desktop app for offline use. Same tools, no browser required."
        />
      </div>
    </section>
  );
}

function FeatureCard({
  icon: Icon,
  title,
  description,
}: {
  icon: LucideIcon;
  title: string;
  description: string;
}) {
  return (
    <div className="flex flex-1 flex-col gap-3">
      <div className="bg-primary/10 text-primary flex size-10 items-center justify-center rounded-lg">
        <Icon className="size-5" />
      </div>
      <h3 className="font-display text-lg font-semibold">{title}</h3>
      <p className="text-muted-foreground text-sm leading-relaxed">
        {description}
      </p>
    </div>
  );
}
