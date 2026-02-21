"use client";

import { isWailsEnvironment } from "../runtime";
import * as convex from "./convex";
import * as wails from "./wails";

const adapter = isWailsEnvironment() ? wails : convex;

export const {
  workflowsQueryOptions,
  workflowQueryOptions,
  executionQueryOptions,
  executionsQueryOptions,
  executionLogsQueryOptions,
  runsRemainingQueryOptions,
  currentUserQueryOptions,
  useSaveWorkflowMutation,
  useRemoveWorkflowMutation,
  useStartExecutionMutation,
} = adapter;
