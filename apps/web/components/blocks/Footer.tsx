import Link from "next/link";

import { ArrowUpRightIcon, GithubIcon } from "@/components/ui/icons";

import { Container } from "@/components/ui/Container";
import { Text } from "@/components/ui/Text";
import { GITHUB_URL, LICENSE_LINE, TAGLINE, TRUST_LINE } from "@/lib/copy";

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
];

export function Footer() {
  return (
    <footer className="border-t border-border">
      <Container>
        <div className="flex flex-col gap-10 py-12 lg:flex-row lg:gap-20 lg:py-16">
          {/* Left column: brand + description */}
          <div className="flex flex-col gap-4 lg:max-w-xs">
            <Link
              href="/"
              className="text-xl font-display font-black tracking-tighter"
            >
              bnto
            </Link>
            <Text size="sm" color="muted" leading="relaxed">
              {TAGLINE} {TRUST_LINE}
            </Text>
            <div className="flex items-center gap-4">
              <a
                href={GITHUB_URL}
                target="_blank"
                rel="noopener noreferrer"
                className="flex items-center gap-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
                aria-label="GitHub repository"
              >
                <GithubIcon className="size-4" />
                Open source
                <ArrowUpRightIcon className="size-3" />
              </a>
            </div>
          </div>

          {/* Right columns: tool links by category */}
          <div className="grid flex-1 grid-cols-2 gap-8 sm:grid-cols-3">
            {TOOL_SECTIONS.map((section) => (
              <div key={section.title}>
                <Text
                  as="h3"
                  size="xs"
                  weight="medium"
                  color="muted"
                  className="mb-3 uppercase tracking-wider"
                >
                  {section.title}
                </Text>
                <ul className="flex flex-col gap-2.5">
                  {section.links.map((link) => (
                    <li key={link.href}>
                      <Link
                        href={link.href}
                        className="text-sm font-medium transition-colors hover:text-primary"
                      >
                        {link.name}
                      </Link>
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        </div>

        {/* Bottom bar */}
        <div className="flex flex-col items-center justify-between gap-3 border-t border-border py-6 text-xs text-muted-foreground sm:flex-row">
          <p>{LICENSE_LINE}</p>
          <Link
            href="/privacy"
            className="transition-colors hover:text-foreground"
          >
            Privacy Policy
          </Link>
        </div>
      </Container>
    </footer>
  );
}
