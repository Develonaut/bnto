"use client";

import { useEffect, useRef } from "react";

import { FadeIn } from "@bnto/ui";

export function FadeLine({
  text,
  className,
  onComplete,
}: {
  text: string;
  className?: string;
  onComplete: () => void;
}) {
  const onCompleteRef = useRef(onComplete);

  useEffect(() => {
    onCompleteRef.current = onComplete;
  });

  useEffect(() => {
    onCompleteRef.current();
  }, []);

  return (
    <FadeIn asChild>
      <span className={className}>{text}</span>
    </FadeIn>
  );
}
