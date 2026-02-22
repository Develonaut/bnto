import Link from "next/link";
import {
  ArrowRightIcon,
  GithubIcon,
  GlobeIcon,
  UserXIcon,
  ZapIcon,
  CodeIcon,
  MonitorIcon,
} from "@bnto/ui";
import type { LucideIcon } from "@bnto/ui";
import { Button } from "@bnto/ui/button";
import { DashedLine } from "@bnto/ui/dashed-line";

export function Hero() {
  return (
    <section className="container flex flex-col items-center gap-12 py-28 lg:flex-row lg:py-32">
      {/* Left column: heading + CTAs */}
      <div className="flex flex-1 flex-col items-center gap-6 text-center lg:items-start lg:text-left">
        <h1 className="text-foreground text-3xl tracking-tight text-balance md:text-4xl lg:text-5xl">
          Free Online Tools for Everyday Tasks
        </h1>
        <p className="text-muted-foreground max-w-xl text-lg leading-relaxed text-balance">
          Compress images, clean CSVs, rename files, and more. No signup, no
          upload limits — just drop your files and go.
        </p>
        <div className="flex flex-wrap items-center gap-3">
          <Button size="lg" asChild>
            <Link href="#tools">
              Explore tools
              <ArrowRightIcon className="ml-1 size-4" />
            </Link>
          </Button>
          <Button variant="outline" size="lg" asChild>
            <a
              href="https://github.com/Develonaut/bnto"
              target="_blank"
              rel="noopener noreferrer"
            >
              <GithubIcon className="mr-1 size-4" />
              View on GitHub
            </a>
          </Button>
        </div>
      </div>

      {/* Separator */}
      <DashedLine orientation="vertical" className="hidden h-64 lg:block" />

      {/* Right column: feature checklist */}
      <div className="flex flex-col gap-4">
        <HeroFeature icon={GlobeIcon} label="Browser-based processing" />
        <HeroFeature icon={UserXIcon} label="No signup required" />
        <HeroFeature icon={ZapIcon} label="No upload limits" />
        <HeroFeature icon={CodeIcon} label="Open source (MIT)" />
        <HeroFeature icon={MonitorIcon} label="Desktop app coming soon" />
      </div>
    </section>
  );
}

function HeroFeature({ icon: Icon, label }: { icon: LucideIcon; label: string }) {
  return (
    <div className="flex items-center gap-3">
      <div className="bg-primary/10 text-primary flex size-9 shrink-0 items-center justify-center rounded-lg">
        <Icon className="size-4" />
      </div>
      <span className="text-sm font-medium">{label}</span>
    </div>
  );
}
