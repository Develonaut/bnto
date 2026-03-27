import Link from "next/link";
import { Stack, Text } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";
import { RECIPES as RECIPE_CATEGORIES } from "../nav/recipeLinks";

interface FooterLink {
  name: string;
  href: string;
  external?: boolean;
}

interface FooterSection {
  title: string;
  links: FooterLink[];
}

/** Recipe categories from the registry + static company links. */
function buildFooterSections(): FooterSection[] {
  const recipeSections: FooterSection[] = RECIPE_CATEGORIES.map((cat) => ({
    title: cat.title,
    links: cat.links.map((link) => ({ name: link.label, href: link.url })),
  }));

  const companySection: FooterSection = {
    title: "Company",
    links: [
      { name: "Pricing", href: "/pricing" },
      { name: "Privacy", href: "/privacy" },
      { name: "GitHub", href: GITHUB_URL, external: true },
    ],
  };

  return [...recipeSections, companySection];
}

const FOOTER_SECTIONS = buildFooterSections();

function externalProps(link: FooterLink) {
  return link.external ? { target: "_blank" as const, rel: "noopener noreferrer" } : {};
}

function FooterSectionColumn({ section }: { section: FooterSection }) {
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
      {FOOTER_SECTIONS.map((section) => (
        <FooterSectionColumn key={section.title} section={section} />
      ))}
    </div>
  );
}
