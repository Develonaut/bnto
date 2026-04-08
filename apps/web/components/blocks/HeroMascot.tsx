import { ScaleIn } from "@bnto/ui";

/** Hero mascot — sushi salmon thumbs-up, anchored bottom-right of terminal. */
export function HeroMascot() {
  return (
    <div
      className="pointer-events-none absolute -right-12 z-10"
      style={{ bottom: "calc(var(--spacing) * -10)" }}
    >
      <ScaleIn from={0.5} easing="spring-bouncy">
        {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
        <img
          src="/mascots/sushi-thumbsup.svg"
          alt=""
          aria-hidden
          className="h-52 w-auto drop-shadow-lg lg:h-64"
        />
      </ScaleIn>
    </div>
  );
}
