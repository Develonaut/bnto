/** Stable pseudo-random delay per slug so cards pop up sporadically. */
export function hashDelay(slug: string): number {
  let hash = 0;
  for (let i = 0; i < slug.length; i++) hash = (hash * 31 + slug.charCodeAt(i)) | 0;
  return Math.abs(hash) % 350;
}
