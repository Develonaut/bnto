/**
 * Result of a feature flag evaluation, including variant and optional payload.
 */
export interface FlagResult {
  key: string;
  enabled: boolean;
  variant: string | boolean | undefined;
  payload: unknown;
}

/**
 * Callback invoked when PostHog flags are loaded or updated.
 */
export type FlagSubscriber = () => void;
