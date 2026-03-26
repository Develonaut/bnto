/**
 * Explore page filters — search input + category pill bar.
 *
 * Reads and writes URL search params (?q=...&category=...).
 */

"use client";

import { useCallback, useTransition } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { getAllCategories, getRecipesByCategory } from "@bnto/registry";
import { Input, Row, SearchIcon } from "@bnto/ui";
import { CATEGORY_ICON } from "@/constants/categoryIcons";
import { CategoryPill } from "./CategoryPill";

/** Categories relevant for recipe filtering (exclude internal-only categories). */
const RECIPE_CATEGORIES = getAllCategories().filter(
  (c) => c.name !== "io" && c.name !== "control" && c.name !== "system",
);

function useExploreParams() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [, startTransition] = useTransition();

  const update = useCallback(
    (key: string, value: string) => {
      const params = new URLSearchParams(searchParams.toString());
      if (!value || value === "all") params.delete(key);
      else params.set(key, value);
      const qs = params.toString();
      startTransition(() => router.replace(`/explore${qs ? `?${qs}` : ""}`, { scroll: false }));
    },
    [router, searchParams, startTransition],
  );

  return {
    query: searchParams.get("q") ?? "",
    category: searchParams.get("category") ?? "all",
    update,
  };
}

export function ExploreFilters() {
  const { query, category, update } = useExploreParams();
  const handleSearch = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => update("q", e.target.value),
    [update],
  );
  const handleCategorySelect = useCallback((c: string) => update("category", c), [update]);

  return (
    <div className="flex flex-col gap-4">
      <div className="relative">
        <SearchIcon className="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          placeholder="Search recipes..."
          defaultValue={query}
          onChange={handleSearch}
          className="pl-9"
          data-testid="explore-search"
        />
      </div>
      <CategoryBar category={category} onSelect={handleCategorySelect} />
    </div>
  );
}

function CategoryBar({ category, onSelect }: { category: string; onSelect: (c: string) => void }) {
  const handleAll = useCallback(() => onSelect("all"), [onSelect]);

  return (
    <Row wrap className="gap-2">
      <CategoryPill label="All" active={category === "all"} onClick={handleAll} />
      {RECIPE_CATEGORIES.map((cat) => (
        <CategoryBarPill
          key={cat.name}
          cat={cat}
          active={category === cat.name}
          onSelect={onSelect}
        />
      ))}
    </Row>
  );
}

function CategoryBarPill({
  cat,
  active,
  onSelect,
}: {
  cat: { name: string; label: string };
  active: boolean;
  onSelect: (c: string) => void;
}) {
  const count = getRecipesByCategory(cat.name).length;
  const handleClick = useCallback(() => onSelect(cat.name), [onSelect, cat.name]);

  return (
    <CategoryPill
      label={cat.label}
      icon={CATEGORY_ICON[cat.name]}
      count={count}
      active={active}
      muted={count === 0}
      onClick={handleClick}
    />
  );
}
