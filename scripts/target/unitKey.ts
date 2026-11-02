/**
 * The unit a file or folder in a cargo profile folder belongs to (ISS-120).
 * Cargo names every unit's outputs, fingerprint and build folder with the
 * same 16-hex-digit hash (`libx-<hash>.rlib`, `.fingerprint/x-<hash>`,
 * `build/x-<hash>`), so the hash is the unit's key.
 */

// The hash ends the name or comes just before its first extension
// (`x-<hash>.x.<cgu hash>-cgu.0.rcgu.o`).
const UNIT_HASH = /-([0-9a-f]{16})(?:\.|$)/;

/** The unit hash in a name, or null when cargo gave it none (copied binaries, `.cargo-lock`). */
export function unitKey(name: string): string | null {
  return UNIT_HASH.exec(name)?.[1] ?? null;
}
