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
  "Save recipes to your account",
  "30-day execution history",
  "AI-powered processing",
  "Server-side video and shell nodes",
  "Team sharing (up to 5 members)",
  "Cloud drive export",
  "API access",
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
        subtitle="Save, collaborate, and run premium nodes."
        features={PRO_FEATURES}
        badge="Coming soon"
        ctaLabel="Coming soon"
        ctaDisabled
        highlight
      />
    </div>
  );
}
