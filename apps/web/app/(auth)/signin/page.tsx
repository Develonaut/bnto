"use client";

import dynamic from "next/dynamic";

const SignInForm = dynamic(
  () => import("./_components/SignInForm").then((m) => m.SignInForm),
  { ssr: false },
);

export default function SignInPage() {
  return <SignInForm />;
}
