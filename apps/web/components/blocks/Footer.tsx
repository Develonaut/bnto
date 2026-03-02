import type { CSSProperties } from "react";

import Link from "next/link";

import { ArrowUpRightIcon, GithubIcon } from "@/components/ui/icons";

import { Container } from "@/components/ui/Container";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { GITHUB_URL, LICENSE_LINE, TAGLINE, TRUST_LINE } from "@/lib/copy";

import { NavButton } from "./NavButton";

const TOOL_SECTIONS = [
  {
    title: "Image",
    links: [
      { name: "Compress Images", href: "/compress-images" },
      { name: "Resize Images", href: "/resize-images" },
      { name: "Convert Format", href: "/convert-image-format" },
    ],
  },
  {
    title: "Data",
    links: [
      { name: "Clean CSV", href: "/clean-csv" },
      { name: "Rename Columns", href: "/rename-csv-columns" },
    ],
  },
  {
    title: "File",
    links: [
      { name: "Rename Files", href: "/rename-files" },
    ],
  },
  {
    title: "Company",
    links: [
      { name: "Pricing", href: "/pricing" },
      { name: "Privacy", href: "/privacy" },
      { name: "GitHub", href: GITHUB_URL, external: true },
    ],
  },
];

export function Footer() {
  return (
    <footer className="border-t border-border">
      <Container>
        <div className="flex flex-col gap-10 py-12 lg:flex-row lg:gap-20 lg:py-16">
          {/* Left column: brand + description */}
          <Stack gap="md" className="lg:max-w-xs">
            <NavButton
              href="/"
              style={{ "--face-bg": "var(--background)", "--face-fg": "var(--foreground)" } as CSSProperties}
              className="w-fit text-xl font-display font-black tracking-tighter"
            >
              bnto
            </NavButton>
            <Text size="sm" color="muted" leading="relaxed">
              {TAGLINE} {TRUST_LINE}
            </Text>
            <Row className="gap-4">
              <a
                href={GITHUB_URL}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
                aria-label="Open source — GitHub repository"
              >
                <GithubIcon className="size-4" />
                Open source
                <ArrowUpRightIcon className="size-3" />
              </a>
            </Row>
          </Stack>

          {/* Right columns: tool links by category */}
          <div className="grid flex-1 grid-cols-2 gap-8 sm:grid-cols-3">
            {TOOL_SECTIONS.map((section) => (
              <div key={section.title}>
                <Text
                  as="p"
                  size="xs"
                  weight="medium"
                  color="muted"
                  className="mb-3 uppercase tracking-wider"
                >
                  {section.title}
                </Text>
                <Stack as="ul" className="gap-2.5">
                  {section.links.map((link) => (
                    <li key={link.href}>
                      <Link
                        href={link.href}
                        className="text-sm font-medium transition-colors hover:text-primary"
                        {...("external" in link && link.external
                          ? { target: "_blank", rel: "noopener noreferrer" }
                          : {})}
                      >
                        {link.name}
                      </Link>
                    </li>
                  ))}
                </Stack>
              </div>
            ))}
          </div>
        </div>

        {/* Bottom bar */}
        <div className="flex flex-col items-center justify-between gap-3 border-t border-border py-6 text-xs text-muted-foreground sm:flex-row">
          <p>{LICENSE_LINE}</p>
          <Row className="gap-4">
            <Link
              href="/motorway"
              className="transition-colors hover:text-foreground"
            >
              Motorway
            </Link>
            <Link
              href="/privacy"
              className="transition-colors hover:text-foreground"
            >
              Privacy Policy
            </Link>
          </Row>
        </div>
      </Container>
    </footer>
  );
}
