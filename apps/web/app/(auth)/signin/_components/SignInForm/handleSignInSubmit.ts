import { core } from "@bnto/core";

interface SignInSubmitArgs {
  isSignUp: boolean;
  name: string;
  email: string;
  password: string;
  signUpEmail: (args: { name: string; email: string; password: string }) => Promise<unknown>;
  signInEmail: (args: { email: string; password: string }) => Promise<unknown>;
}

/** Execute the sign-in or sign-up flow. Clears the signout signal first. */
export async function handleSignInSubmit(args: SignInSubmitArgs): Promise<void> {
  core.auth.clearSignoutSignal();
  if (args.isSignUp) {
    await args.signUpEmail({ name: args.name, email: args.email, password: args.password });
  } else {
    await args.signInEmail({ email: args.email, password: args.password });
  }
}
