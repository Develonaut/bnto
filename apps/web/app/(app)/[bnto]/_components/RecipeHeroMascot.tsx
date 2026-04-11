import { ScaleIn } from "@bnto/ui";
import { getCategoryMascot } from "../_utils/categoryMascot";

/** Mascot anchored bottom-right of the dropzone, overlapping slightly. */
export function RecipeHeroMascot({ category }: { category: string }) {
  const src = getCategoryMascot(category);
  return (
    <div
      className="pointer-events-none absolute right-2 z-10"
      style={{ bottom: "calc(var(--spacing) * -10)" }}
    >
      <ScaleIn from={0.5} easing="spring-bouncy">
        {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
        <img src={src} alt="" aria-hidden className="h-32 w-auto drop-shadow-lg sm:h-40" />
      </ScaleIn>
    </div>
  );
}
