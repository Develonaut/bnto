import Link from "next/link";

import { Stack, Text } from "@bnto/ui";

interface FooterLink {
  name: string;
  href: string;
  external?: boolean;
}

export interface FooterSection {
  title: string;
  links: FooterLink[];
}

const externalProps = (link: FooterLink) =>
  link.external ? { target: "_blank" as const, rel: "noopener noreferrer" } : {};

export function FooterSectionColumn({ section }: { section: FooterSection }) {
  return (
    <div>
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
              data-testid={`footer-link-${link.href.replace("/", "")}`}
              className="text-sm font-medium transition-colors hover:text-primary"
              {...externalProps(link)}
            >
              {link.name}
            </Link>
          </li>
        ))}
      </Stack>
    </div>
  );
}
