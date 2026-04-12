/** Mascot assignments for below-the-fold SEO sections. */

import { getCategoryMascot } from "./categoryMascot";

/** Step mascots for the "How it works" section. Step 2 varies by category. */
export function getStepMascots(category: string) {
  return [
    "/mascots/sushi-friends.svg",
    getCategoryMascot(category),
    "/mascots/sushi-motorbike.svg",
  ] as const;
}

/** Trust section mascot (shared across all recipes). */
export const TRUST_MASCOT = "/mascots/sumo-sushi.svg";

/** FAQ section mascot (shared across all recipes). */
export const FAQ_MASCOT = "/mascots/sushi-onigiri.svg";
