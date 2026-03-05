/**
 * Format version constants for `.bnto.json` definitions.
 *
 * The format version tracks the schema of the Definition structure itself —
 * not individual node parameter schemas (those have `schemaVersion` on NodeSchema).
 *
 * Semver rules:
 * - MAJOR: breaking changes to Definition structure (new required fields, removed fields, restructured nesting)
 * - MINOR: additive changes (new optional fields, new node types)
 * - PATCH: documentation or metadata-only changes
 *
 * Compatibility: engines check major version match. A definition with version "1.3.0"
 * runs on an engine that supports "1.0.0" — minor/patch differences are forward-compatible.
 */

/** All format versions this engine supports (newest last). */
export const SUPPORTED_FORMAT_VERSIONS = ["1.0.0"] as const;

/** The current format version for new definitions — always the latest supported. */
export const CURRENT_FORMAT_VERSION = SUPPORTED_FORMAT_VERSIONS[
  SUPPORTED_FORMAT_VERSIONS.length - 1
] as (typeof SUPPORTED_FORMAT_VERSIONS)[number];

/** Checks if a version string is in the supported versions list. */
export function isSupportedVersion(version: string): boolean {
  return (SUPPORTED_FORMAT_VERSIONS as readonly string[]).includes(version);
}

/**
 * Checks if a version is compatible (same major version as any supported version).
 *
 * A definition with version "1.3.0" is compatible with an engine that supports "1.0.0"
 * because the major version matches. "2.0.0" would not be compatible.
 */
export function isCompatibleVersion(version: string): boolean {
  const major = parseMajor(version);
  if (major === null) return false;
  return SUPPORTED_FORMAT_VERSIONS.some((sv) => parseMajor(sv) === major);
}

/** Extracts the major version number from a valid semver string. Returns null for invalid strings. */
function parseMajor(version: string): number | null {
  const match = version.match(/^(\d+)\.(\d+)\.(\d+)$/);
  if (!match) return null;
  return parseInt(match[1], 10);
}
