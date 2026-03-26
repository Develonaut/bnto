"use client";

import type { ChangeEvent } from "react";
import { useCallback, useState } from "react";

/** Manages the form field state for sign-in / sign-up. */
export function useSignInFormFields(initialEmail: string) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState(initialEmail);
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const onNameChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => setName(e.target.value),
    [],
  );
  const onEmailChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => setEmail(e.target.value),
    [],
  );
  const onPasswordChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => setPassword(e.target.value),
    [],
  );

  return {
    name,
    email,
    password,
    error,
    loading,
    setEmail,
    setError,
    setLoading,
    onNameChange,
    onEmailChange,
    onPasswordChange,
  };
}
