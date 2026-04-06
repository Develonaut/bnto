"use client";

import { useCallback, useEffect, useRef, useState } from "react";

/** Track whether an element has scrolled into view (trigger-once, optional stagger delay). */
export function useStepInView(threshold = 0.4, delay = 0) {
  const [inView, setInView] = useState(false);
  const elRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const el = elRef.current;
    if (!el || inView) return;
    let timer: ReturnType<typeof setTimeout>;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (!entry.isIntersecting) return;
        observer.disconnect();
        if (delay > 0) timer = setTimeout(() => setInView(true), delay);
        else setInView(true);
      },
      { threshold, rootMargin: "-10% 0px" },
    );
    observer.observe(el);
    return () => {
      observer.disconnect();
      clearTimeout(timer);
    };
  }, [inView, threshold, delay]);

  const ref = useCallback((node: HTMLDivElement | null) => {
    elRef.current = node;
  }, []);

  return [inView, ref] as const;
}
