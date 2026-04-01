"use client";

import { useEffect, useState } from "react";

/** Derive an object URL + aspect ratio from the first user-uploaded file. */
export function useSourcePreview(files: File[]) {
  const [sourceUrl, setSourceUrl] = useState("");
  const [aspectRatio, setAspectRatio] = useState(1);

  useEffect(() => {
    const file = files[0];
    if (!file) {
      setSourceUrl("");
      return;
    }
    const url = URL.createObjectURL(file);
    setSourceUrl(url);
    const img = new Image();
    img.onload = () => setAspectRatio(img.naturalWidth / img.naturalHeight);
    img.src = url;
    return () => URL.revokeObjectURL(url);
  }, [files]);

  return { sourceUrl, aspectRatio };
}
