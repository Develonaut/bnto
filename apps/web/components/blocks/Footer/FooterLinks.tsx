import { GITHUB_URL } from "@/lib/links";
import { RECIPES as RECIPE_CATEGORIES } from "../nav/recipeLinks";
import { FooterSectionColumn, type FooterSection } from "./FooterSectionColumn";

/** Recipe categories from the registry + static company links. */
const buildFooterSections = (): FooterSection[] => {
  const recipeSections: FooterSection[] = RECIPE_CATEGORIES.map((cat) => ({
    title: cat.title,
    links: cat.links.map((link) => ({ name: link.label, href: link.url })),
  }));

  const companySection: FooterSection = {
    title: "Company",
    links: [
      { name: "FAQ", href: "/faq" },
      { name: "Privacy", href: "/privacy" },
      { name: "GitHub", href: GITHUB_URL, external: true },
    ],
  };

  return [...recipeSections, companySection];
};

const FOOTER_SECTIONS = buildFooterSections();

export function FooterLinks() {
  return (
    <div className="grid flex-1 grid-cols-2 gap-8 sm:grid-cols-3">
      {FOOTER_SECTIONS.map((section) => (
        <FooterSectionColumn key={section.title} section={section} />
      ))}
    </div>
  );
}
