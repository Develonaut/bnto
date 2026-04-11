import { ScaleIn } from "@bnto/ui";
import { getCategoryMascot } from "../_utils/categoryMascot";

/** Centered mascot above the recipe H1, matched to recipe category. */
export function RecipeHeroMascot({ category }: { category: string }) {
  const src = getCategoryMascot(category);
  return (
    <div className="flex justify-center">
      <ScaleIn from={0.5} easing="spring-bouncy">
        {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
        <img src={src} alt="" aria-hidden className="h-28 w-auto drop-shadow-lg sm:h-36" />
      </ScaleIn>
    </div>
  );
}
