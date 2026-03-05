/**
 * HTTP Request node schema — parameters for making HTTP requests.
 *
 * Go source: engine/pkg/node/library/http/http.go
 * Validator: engine/pkg/validator/validators.go → validateHTTPRequest
 */

import type { NodeSchema } from "./types";

/** Valid HTTP methods accepted by the http-request node. */
export const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"] as const;

export const httpRequestSchema: NodeSchema = {
  nodeType: "http-request",
  schemaVersion: 1,
  parameters: [
    {
      name: "url",
      type: "string",
      required: true,
      label: "URL",
      description: "The URL to send the request to.",
      placeholder: "https://api.example.com/endpoint",
    },
    {
      name: "method",
      type: "enum",
      required: true,
      label: "Method",
      description: "HTTP method to use for the request.",
      default: "GET",
      enumValues: HTTP_METHODS,
    },
    {
      name: "headers",
      type: "object",
      required: false,
      label: "Headers",
      description: "Custom HTTP headers to include in the request.",
    },
    {
      name: "body",
      type: "object",
      required: false,
      label: "Body",
      description: "JSON request body (automatically serialized).",
    },
    {
      name: "queryParams",
      type: "object",
      required: false,
      label: "Query Parameters",
      description: "URL query parameters to append to the URL.",
    },
    {
      name: "timeout",
      type: "number",
      required: false,
      label: "Timeout",
      description: "Request timeout in seconds.",
      default: 30,
      min: 1,
      max: 3600,
    },
    {
      name: "saveToFile",
      type: "string",
      required: false,
      label: "Save to File",
      description: "File path to save the response body (skips JSON parsing).",
      placeholder: "/path/to/output.json",
    },
  ],
} as const;
