/**
 * Animate a theme change using the View Transitions API.
 * Renders a circular clip-path reveal expanding from the button center.
 * Falls back to instant switch if View Transitions or reduced motion.
 */
export async function animateThemeTransition(
  buttonEl: HTMLButtonElement,
  applyTheme: () => void,
) {
  // Reduced motion -- instant switch
  if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
    applyTheme();
    return;
  }

  // Fallback for browsers without View Transitions API
  if (!document.startViewTransition) {
    applyTheme();
    return;
  }

  const transition = document.startViewTransition(() => {
    applyTheme();
  });

  await transition.ready;

  const { top, left, width, height } = buttonEl.getBoundingClientRect();
  const x = left + width / 2;
  const y = top + height / 2;

  const maxRadius = Math.hypot(
    Math.max(x, window.innerWidth - x),
    Math.max(y, window.innerHeight - y),
  );

  document.documentElement.animate(
    {
      clipPath: [
        `circle(0px at ${x}px ${y}px)`,
        `circle(${maxRadius}px at ${x}px ${y}px)`,
      ],
    },
    {
      duration: 500,
      easing: "ease-in-out",
      pseudoElement: "::view-transition-new(root)",
    },
  );
}
