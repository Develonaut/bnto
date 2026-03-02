/**
 * Configuration for initializing the telemetry adapter (PostHog).
 */
export interface TelemetryConfig {
  apiKey: string;
  host: string;
}

/**
 * Properties bag passed alongside telemetry events.
 *
 * Keep this open — PostHog accepts arbitrary key/value pairs.
 * Known properties are typed for autocomplete; extras pass through.
 */
export interface TelemetryProperties {
  /** The bnto recipe slug (e.g., "compress-images") */
  slug?: string;
  /** Whether the user is authenticated */
  authenticated?: boolean;
  /** Arbitrary extra properties */
  [key: string]: unknown;
}

/**
 * User traits sent on identify calls.
 */
export interface TelemetryUserTraits {
  email?: string;
  name?: string;
  [key: string]: unknown;
}
