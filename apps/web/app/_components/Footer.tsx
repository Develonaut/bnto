import Link from "next/link";
import { ArrowUpRightIcon } from "@bnto/ui";
import { Background } from "@bnto/ui/background";
import { DashedLine } from "@bnto/ui/dashed-line";
import { FOOTER_SECTIONS } from "./footer-links";
import type { FooterLink } from "./footer-links";

/**
 * Brand footer with CTA gradient, nav link grid, and wordmark.
 *
 * Server component -- no hooks needed. Importable from any layout.
 */
export function Footer() {
  return (
    <footer>
      {/* CTA section */}
      <Background variant="bottom" className="px-6 py-16 text-center lg:px-8">
        <h2 className="font-display text-3xl font-bold tracking-tight text-foreground sm:text-4xl">
          Get things done, faster
        </h2>
        <p className="mx-auto mt-4 max-w-md text-muted-foreground">
          Free online tools for everyday tasks. No signup, no upload limits,
          no nonsense.
        </p>
        <div className="mt-8">
          <Link
            href="/"
            className="inline-flex items-center gap-2 rounded-full bg-primary px-6 py-3 text-sm font-medium text-primary-foreground motion-safe:transition-colors hover:bg-primary/90"
          >
            Explore tools
          </Link>
        </div>
      </Background>

      <DashedLine className="mx-6 lg:mx-8" />

      {/* Nav links + wordmark */}
      <div className="px-6 py-12 lg:px-8">
        <div className="mx-auto max-w-5xl">
          <div className="grid gap-8 sm:grid-cols-3">
            {FOOTER_SECTIONS.map((section) => (
              <div key={section.title}>
                <h3 className="text-sm font-semibold text-foreground">
                  {section.title}
                </h3>
                <ul className="mt-3 space-y-2">
                  {section.links.map((link) => (
                    <li key={link.href}>
                      <FooterLink link={link} />
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </div>

          <div className="mt-16 flex items-end justify-between">
            <span className="font-display text-6xl font-bold tracking-tight text-muted-foreground/30 sm:text-8xl">
              bnto
            </span>
            <p className="text-xs text-muted-foreground">
              &copy; {new Date().getFullYear()} bnto
            </p>
          </div>
        </div>
      </div>
    </footer>
  );
}

function FooterLink({ link }: { link: FooterLink }) {
  if (link.external) {
    return (
      <a
        href={link.href}
        target="_blank"
        rel="noopener noreferrer"
        className="inline-flex items-center gap-1 text-sm text-muted-foreground motion-safe:transition-colors hover:text-foreground"
      >
        {link.label}
        <ArrowUpRightIcon className="size-3" />
      </a>
    );
  }
  return (
    <Link
      href={link.href}
      className="text-sm text-muted-foreground motion-safe:transition-colors hover:text-foreground"
    >
      {link.label}
    </Link>
  );
}
