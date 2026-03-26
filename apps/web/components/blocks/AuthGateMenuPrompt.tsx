"use client";

import type { ReactNode } from "react";

import { Menu, MenuContent, PopoverAnchor } from "@bnto/ui";

import { GateTrigger } from "./GateTrigger";
import { AuthGateMenuBody } from "./AuthGateMenuBody";

interface AuthGateMenuPromptProps {
  open: boolean;
  setOpen: (open: boolean) => void;
  handleGateClick: (e: React.MouseEvent) => void;
  handleGateKeyDown: (e: React.KeyboardEvent) => void;
  title: string;
  description: string;
  children: ReactNode;
}

export function AuthGateMenuPrompt({
  open,
  setOpen,
  handleGateClick,
  handleGateKeyDown,
  title,
  description,
  children,
}: AuthGateMenuPromptProps) {
  return (
    <Menu open={open} onOpenChange={setOpen}>
      <PopoverAnchor asChild>
        <GateTrigger onClickCapture={handleGateClick} onKeyDownCapture={handleGateKeyDown}>
          {children}
        </GateTrigger>
      </PopoverAnchor>
      <MenuContent side="top" align="center" className="w-72 p-5">
        <AuthGateMenuBody title={title} description={description} />
      </MenuContent>
    </Menu>
  );
}
