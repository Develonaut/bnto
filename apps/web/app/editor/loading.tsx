import { EditorLoadingSkeleton } from "./_components/EditorLoadingSkeleton";

/** Route-level loading fallback — Suspense boundary for useSearchParams(). */
export default function EditorLoading() {
  return <EditorLoadingSkeleton />;
}
