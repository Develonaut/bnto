/**
 * HTTP Request node schema — parameters for making HTTP requests.
 *
 * Go source: engine/pkg/node/library/http/http.go
 * Validator: engine/pkg/validator/validators.go → validateHTTPRequest
 */

import { z } from "zod";
import type { NodeSchemaDefinition } from "./types";

/** Valid HTTP methods accepted by the http-request node. */
export const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"] as const;

/** Zod schema for http-request node parameters. */
export const httpRequestParamsSchema = z.object({
  url: z.string(),
  method: z.enum(HTTP_METHODS).default("GET"),
  headers: z.record(z.string()).optional(),
  body: z.record(z.unknown()).optional(),
  queryParams: z.record(z.string()).optional(),
  timeout: z.number().min(1).max(3600).optional().default(30),
  saveToFile: z.string().optional(),
});

/** Inferred TypeScript type for http-request node parameters. */
export type HttpRequestParams = z.infer<typeof httpRequestParamsSchema>;

/** Full schema definition for the http-request node type. */
export const httpRequestNodeSchema: NodeSchemaDefinition = {
  nodeType: "http-request",
  schemaVersion: 1,
  schema: httpRequestParamsSchema,
  params: {
    url: {
      label: "URL",
      description: "The URL to send the request to.",
      placeholder: "https://api.example.com/endpoint",
    },
    method: {
      label: "Method",
      description: "HTTP method to use for the request.",
    },
    headers: {
      label: "Headers",
      description: "Custom HTTP headers to include in the request.",
    },
    body: {
      label: "Body",
      description: "JSON request body (automatically serialized).",
    },
    queryParams: {
      label: "Query Parameters",
      description: "URL query parameters to append to the URL.",
    },
    timeout: {
      label: "Timeout",
      description: "Request timeout in seconds.",
    },
    saveToFile: {
      label: "Save to File",
      description: "File path to save the response body (skips JSON parsing).",
      placeholder: "/path/to/output.json",
    },
  },
};
