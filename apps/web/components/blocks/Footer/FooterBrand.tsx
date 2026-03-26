import type { CSSProperties } from "react";
import { Stack, Text } from "@bnto/ui";
import { TAGLINE, TRUST_LINE } from "@/lib/copy";
import { NavButton } from "../NavButton";
import { FooterBrandLinks } from "./FooterBrandLinks";

export function FooterBrand() {
  return (
    <Stack gap="md" className="lg:max-w-xs">
      <NavButton
        href="/"
        style={
          { "--face-bg": "var(--background)", "--face-fg": "var(--foreground)" } as CSSProperties
        }
        className="w-fit text-xl font-display font-black tracking-tighter"
      >
        bnto
      </NavButton>
      <Text size="sm" color="muted" leading="relaxed">
        {TAGLINE} {TRUST_LINE}
      </Text>
      <FooterBrandLinks />
    </Stack>
  );
}
