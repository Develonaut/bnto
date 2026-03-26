type Mode = "signin" | "signup";

interface ToggleSignInModeResult {
  nextMode: Mode;
  nextEmail: string;
}

/** Pure function — computes the next mode and email after toggling sign-in / sign-up. */
export function toggleSignInMode(
  currentIsSignUp: boolean,
  existingEmail: string | undefined,
): ToggleSignInModeResult {
  const nextMode: Mode = currentIsSignUp ? "signin" : "signup";
  const nextEmail = nextMode === "signin" ? (existingEmail ?? "") : "";
  return { nextMode, nextEmail };
}
