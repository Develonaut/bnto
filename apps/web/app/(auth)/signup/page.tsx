"use client";

import dynamic from "next/dynamic";

const SignInForm = dynamic(
  () =>
    import("../signin/_components/SignInForm").then((m) => m.SignInForm),
  { ssr: false },
);

export default function SignUpPage() {
  return <SignInForm defaultMode="signup" />;
}
