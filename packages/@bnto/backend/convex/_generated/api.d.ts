/* eslint-disable */
/**
 * Generated `api` utility.
 *
 * THIS CODE IS AUTOMATICALLY GENERATED.
 *
 * To regenerate, run `npx convex dev`.
 * @module
 */

import type * as _helpers_auth from "../_helpers/auth.js";
import type * as _helpers_quota from "../_helpers/quota.js";
import type * as _helpers_r2_client from "../_helpers/r2_client.js";
import type * as _helpers_run_limits from "../_helpers/run_limits.js";
import type * as _helpers_upload_validation from "../_helpers/upload_validation.js";
import type * as _test_helpers from "../_test_helpers.js";
import type * as auth from "../auth.js";
import type * as cleanup from "../cleanup.js";
import type * as cleanup_stale from "../cleanup_stale.js";
import type * as crons from "../crons.js";
import type * as downloads from "../downloads.js";
import type * as executionLogs from "../executionLogs.js";
import type * as execution_events from "../execution_events.js";
import type * as executions from "../executions.js";
import type * as http from "../http.js";
import type * as uploads from "../uploads.js";
import type * as users from "../users.js";
import type * as workflows from "../workflows.js";

import type {
  ApiFromModules,
  FilterApi,
  FunctionReference,
} from "convex/server";

declare const fullApi: ApiFromModules<{
  "_helpers/auth": typeof _helpers_auth;
  "_helpers/quota": typeof _helpers_quota;
  "_helpers/r2_client": typeof _helpers_r2_client;
  "_helpers/run_limits": typeof _helpers_run_limits;
  "_helpers/upload_validation": typeof _helpers_upload_validation;
  _test_helpers: typeof _test_helpers;
  auth: typeof auth;
  cleanup: typeof cleanup;
  cleanup_stale: typeof cleanup_stale;
  crons: typeof crons;
  downloads: typeof downloads;
  executionLogs: typeof executionLogs;
  execution_events: typeof execution_events;
  executions: typeof executions;
  http: typeof http;
  uploads: typeof uploads;
  users: typeof users;
  workflows: typeof workflows;
}>;

/**
 * A utility for referencing Convex functions in your app's public API.
 *
 * Usage:
 * ```js
 * const myFunctionReference = api.myModule.myFunction;
 * ```
 */
export declare const api: FilterApi<
  typeof fullApi,
  FunctionReference<any, "public">
>;

/**
 * A utility for referencing Convex functions in your app's internal API.
 *
 * Usage:
 * ```js
 * const myFunctionReference = internal.myModule.myFunction;
 * ```
 */
export declare const internal: FilterApi<
  typeof fullApi,
  FunctionReference<any, "internal">
>;

export declare const components: {};
