"use client";

import type { ReactNode } from "react";

import { Dialog } from "@bnto/ui";

import { GateTrigger } from "./GateTrigger";
import { AuthGateDialogBody } from "./AuthGateDialogBody";

interface AuthGateDialogPromptProps {
  open: boolean;
  setOpen: (open: boolean) => void;
  handleGateClick: (e: React.MouseEvent) => void;
  handleGateKeyDown: (e: React.KeyboardEvent) => void;
  title: string;
  description: string;
  children: ReactNode;
}

export function AuthGateDialogPrompt({
  open,
  setOpen,
  handleGateClick,
  handleGateKeyDown,
  title,
  description,
  children,
}: AuthGateDialogPromptProps) {
  return (
    <>
      <GateTrigger onClickCapture={handleGateClick} onKeyDownCapture={handleGateKeyDown}>
        {children}
      </GateTrigger>
      <Dialog open={open} onOpenChange={setOpen}>
        <AuthGateDialogBody title={title} description={description} />
      </Dialog>
    </>
  );
}
