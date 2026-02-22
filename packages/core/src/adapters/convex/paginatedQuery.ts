"use client";

/**
 * Single import boundary for Convex's native pagination hook.
 *
 * Hooks import usePaginatedQuery through this file, never directly
 * from "convex/react". This keeps the Convex dependency isolated.
 */
export { usePaginatedQuery } from "convex/react";
