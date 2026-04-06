"use client";

import { useEffect, useRef, useState } from "react";

export function TypingLine({
  text,
  speed = 40,
  className,
  onComplete,
}: {
  text: string;
  speed?: number;
  className?: string;
  onComplete: () => void;
}) {
  const [displayed, setDisplayed] = useState("");
  const indexRef = useRef(0);
  const onCompleteRef = useRef(onComplete);

  useEffect(() => {
    onCompleteRef.current = onComplete;
  });

  useEffect(() => {
    const interval = setInterval(() => {
      indexRef.current += 1;
      if (indexRef.current > text.length) {
        clearInterval(interval);
        onCompleteRef.current();
        return;
      }
      setDisplayed(text.slice(0, indexRef.current));
    }, speed);
    return () => clearInterval(interval);
  }, [text, speed]);

  return (
    <span className={className}>
      {displayed}
      {displayed.length < text.length && <span className="motion-safe:animate-pulse">▎</span>}
    </span>
  );
}
