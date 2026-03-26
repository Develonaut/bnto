/** Run a 0 -> target count animation using requestAnimationFrame. */
export function runCountAnimation(
  el: HTMLSpanElement,
  target: number,
  duration: number,
): (() => void) | undefined {
  const prefersReduced =
    typeof window !== "undefined" && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  if (prefersReduced) {
    el.textContent = String(target);
    return;
  }

  const start = performance.now();
  let rafId: number | null = null;

  const tick = (now: number) => {
    const elapsed = now - start;
    const progress = Math.min(elapsed / duration, 1);
    // Ease-out cubic: fast start, gentle settle
    const eased = 1 - Math.pow(1 - progress, 3);
    el.textContent = String(Math.round(eased * target));
    if (progress < 1) {
      rafId = requestAnimationFrame(tick);
    }
  };

  rafId = requestAnimationFrame(tick);
  return () => {
    if (rafId) cancelAnimationFrame(rafId);
  };
}
