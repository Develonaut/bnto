"use client";

import { useState, useRef, useEffect, useCallback } from "react";
import type { Ref, MutableRefObject } from "react";

/** Start observing an element and call setInView(true) when visible. */
function observeElement(
  el: HTMLDivElement,
  threshold: number,
  triggerOnce: boolean,
  setInView: (v: boolean) => void,
) {
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
}

/**
 * Track whether an element is in view via IntersectionObserver.
 * Returns `[inView, setRef]` for use in forwardRef components.
 */
export function useIntersectionVisible(
  threshold: number,
  triggerOnce: boolean,
  forwardedRef: Ref<HTMLDivElement>,
) {
  const [inView, setInView] = useState(() => {
    if (typeof window === "undefined") return false;
    return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  });
  const elRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (inView) return;
    const el = elRef.current;
    if (!el) return;
    return observeElement(el, threshold, triggerOnce, setInView);
  }, [inView, threshold, triggerOnce]);

  const setRef = useCallback(
    (node: HTMLDivElement | null) => {
      elRef.current = node;
      if (typeof forwardedRef === "function") forwardedRef(node);
      else if (forwardedRef)
        (forwardedRef as MutableRefObject<HTMLDivElement | null>).current = node;
    },
    [forwardedRef],
  );

  return [inView, setRef] as const;
}
