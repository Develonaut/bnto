/**
 * Static page navigation links for navbar and mobile menu.
 */

export interface PageLink {
  label: string;
  href: string;
}

export const PAGE_LINKS: PageLink[] = [
  { label: "Pricing", href: "/pricing" },
  { label: "FAQ", href: "/faq" },
];
