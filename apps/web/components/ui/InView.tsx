"use client";

import * as React from "react";

import { Slot } from "@radix-ui/react-slot";

/* ── InView ──────────────────────────────────────────────────── */

interface InViewProps extends React.HTMLAttributes<HTMLDivElement> {
  asChild?: boolean;
  /** IntersectionObserver threshold (0-1). Default 0.15. */
  threshold?: number;
  /** Only trigger once — stays visible after first intersection. Default true. */
  triggerOnce?: boolean;
}

export const InView = React.forwardRef<HTMLDivElement, InViewProps>(
  ({ threshold = 0.15, triggerOnce = true, asChild, className, style, children, ...props }, ref) => {
    const [inView, setInView] = React.useState(false);
    const elRef = React.useRef<HTMLDivElement | null>(null);

    React.useEffect(() => {
      const el = elRef.current;
      if (!el) return;

      const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
      if (prefersReduced) {
        setInView(true);
        return;
      }

      const observer = new IntersectionObserver(
        ([entry]) => {
          if (entry.isIntersecting) {
            setInView(true);
            if (triggerOnce) observer.disconnect();
          }
        },
        { threshold, rootMargin: "-15% 0px" },
      );
      observer.observe(el);
      return () => observer.disconnect();
    }, [threshold, triggerOnce]);

    const setRef = React.useCallback(
      (node: HTMLDivElement | null) => {
        elRef.current = node;
        if (typeof ref === "function") ref(node);
        else if (ref) (ref as React.MutableRefObject<HTMLDivElement | null>).current = node;
      },
      [ref],
    );

    const Comp = asChild ? Slot : "div";
    return (
      <Comp
        ref={setRef}
        data-in-view={inView}
        className={className}
        style={style}
        {...props}
      >
        {children}
      </Comp>
    );
  },
);
InView.displayName = "InView";
