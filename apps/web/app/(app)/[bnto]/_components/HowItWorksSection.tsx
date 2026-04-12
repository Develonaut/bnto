import type { Recipe } from "@bnto/core";
import { SectionHeader, SectionShell, StepFlowLayout } from "@/components/sections";
import type { StepFlowStep } from "@/components/sections";
import { getHowItWorks } from "@/lib/recipePageContent";
import { getStepMascots } from "../_utils/sectionMascots";

const STEP_VARIANTS = ["primary", "secondary", "accent"] as const;

export function HowItWorksSection({ recipe }: { recipe: Recipe }) {
  const rawSteps = getHowItWorks(recipe);
  const mascots = getStepMascots(recipe.category);

  const steps: StepFlowStep[] = rawSteps.map((s, i) => ({
    step: i + 1,
    title: s.title,
    description: s.description,
    mascot: mascots[i],
    mascotHeight: 140,
    variant: STEP_VARIANTS[i],
  }));

  return (
    <SectionShell>
      <SectionHeader title="Three steps. No signup." />
      <StepFlowLayout steps={steps} />
    </SectionShell>
  );
}
