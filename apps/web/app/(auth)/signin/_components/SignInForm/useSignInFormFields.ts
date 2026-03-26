"use client";

import type { ChangeEvent } from "react";
import { useCallback, useState } from "react";

function useInputHandler(setter: (value: string) => void) {
  return useCallback((e: ChangeEvent<HTMLInputElement>) => setter(e.target.value), [setter]);
}

/** Manages the form field state for sign-in / sign-up. */
export function useSignInFormFields(initialEmail: string) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState(initialEmail);
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const onNameChange = useInputHandler(setName);
  const onEmailChange = useInputHandler(setEmail);
  const onPasswordChange = useInputHandler(setPassword);

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
