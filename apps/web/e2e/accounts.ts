/**
 * Shared test account constants and helpers for E2E tests.
 *
 * Predefined accounts provide stable identities for tests that need auth
 * without going through the signup flow. The `testEmail()` function generates
 * unique throwaway emails for signup-specific tests.
 *
 * All test emails use @test.bnto.dev — the cleanup mutation deletes these
 * after every test run.
 */

export const TEST_PASSWORD = "TestPassword123!";
export const TEST_NAME = "E2E Test User";
export const TEST_EMAIL_DOMAIN = "@test.bnto.dev";

/** Predefined accounts for tests that need auth without testing signup. */
export const TEST_ACCOUNTS = {
  basic: { email: "e2e-basic@test.bnto.dev", name: "E2E Basic User" },
  pro: { email: "e2e-pro@test.bnto.dev", name: "E2E Pro User" },
} as const;

/** Generate a unique test email (for signup-specific tests). */
export function testEmail() {
  const id = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
  return `e2e-${id}@test.bnto.dev`;
}
