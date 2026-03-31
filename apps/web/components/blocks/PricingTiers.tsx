import { PricingTierCard } from "./PricingTierCard";

const FREE_FEATURES = [
  "Unlimited image compression, resizing, and conversion",
  "Unlimited CSV cleaning and column renaming",
  "Unlimited batch file renaming",
  "Visual recipe editor — build custom multi-step recipes",
  "Process multiple files at once",
  "No account required",
  "Files never leave your browser",
  "Open source engine (MIT)",
];

const PRO_FEATURES = [
  "Everything in Free",
  "Save and manage recipes in the cloud",
  "AI-powered processing (Claude, GPT)",
  "Server-side video and audio nodes",
  "Shell command execution (ffmpeg, ImageMagick)",
];

export function PricingTiers() {
  return (
    <div className="mx-auto mt-12 grid max-w-3xl grid-cols-1 gap-6 md:grid-cols-2">
      <PricingTierCard
        name="Free"
        price="$0"
        subtitle="Forever. No strings attached."
        features={FREE_FEATURES}
        ctaLabel="Start using bnto"
        ctaHref="/"
      />
      <PricingTierCard
        name="Pro"
        price="TBD"
        subtitle="Server-powered processing for what browsers can't do."
        features={PRO_FEATURES}
        badge="Coming soon"
        ctaLabel="Coming soon"
        ctaDisabled
        highlight
      />
    </div>
  );
}
