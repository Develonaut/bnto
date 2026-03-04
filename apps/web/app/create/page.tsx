import { RecipeEditor } from "@/components/editor/RecipeEditor";

/**
 * /create — full-viewport recipe editor.
 *
 * Server component page — RecipeEditor has its own "use client" boundary.
 */
export default function CreatePage() {
  return <RecipeEditor />;
}
