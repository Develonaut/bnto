import Link from "next/link";
import { Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";

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
    links: [{ name: "Rename Files", href: "/rename-files" }],
  },
  {
    title: "Company",
    links: [
      { name: "Pricing", href: "/pricing" },
      { name: "Privacy", href: "/privacy" },
      { name: "GitHub", href: GITHUB_URL, external: true },
    ],
  },
] as const;

type FooterLink = (typeof TOOL_SECTIONS)[number]["links"][number];

function externalProps(link: FooterLink) {
  return "external" in link && link.external
    ? { target: "_blank" as const, rel: "noopener noreferrer" }
    : {};
}

function FooterSection({ section }: { section: (typeof TOOL_SECTIONS)[number] }) {
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

export function FooterLinks() {
  return (
    <div className="grid flex-1 grid-cols-2 gap-8 sm:grid-cols-3">
      {TOOL_SECTIONS.map((section) => (
        <FooterSection key={section.title} section={section} />
      ))}
    </div>
  );
}
