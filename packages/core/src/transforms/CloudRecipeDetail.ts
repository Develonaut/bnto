/** Cloud recipe detail -- internal shape for Convex detail queries. */
import type { RawRecipeDoc } from "../types/raw";

export interface CloudRecipeDetail {
  id: string;
  userId: string;
  name: string;
  definition: RawRecipeDoc["definition"];
  version: number;
  formatVersion?: string;
  isPublic: boolean;
  createdAt: number;
  updatedAt: number;
}
