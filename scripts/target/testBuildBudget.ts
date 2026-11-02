/**
 * How big one build of the engine's tests (`cargo test --workspace`) may get,
 * per OS, in MB (10^6 bytes): its units' outputs, objects and build-script
 * folders, not incremental caches (ISS-120). Each is the size measured when
 * it was set, plus about 15%. Raise one in a commit that says what grew.
 */
export const TEST_BUILD_BUDGET_MB: Partial<Record<NodeJS.Platform, number>> = {
  // 1366 MB, November 2, 2026.
  darwin: 1600,
  // Measured on CI's ubuntu-latest, November 2, 2026.
  linux: 99999,
  // Measured on CI's windows-latest, November 2, 2026.
  win32: 99999,
};
