import Link from "next/link";
import { ArrowUpRightIcon } from "@bnto/ui";
import { Logo } from "@bnto/ui/logo";
import { Button } from "@bnto/ui/button";

export function Footer() {
  return (
    <footer className="flex flex-col items-center gap-14 pt-28 lg:pt-32">
      <div className="container space-y-3 text-center">
        <h2 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
          Get things done, faster
        </h2>
        <p className="text-muted-foreground mx-auto max-w-xl leading-snug text-balance">
          Free online tools for everyday tasks. No signup, no upload limits, no
          nonsense.
        </p>
        <div>
          <Button size="lg" className="mt-4" asChild>
            <Link href="/">Explore tools</Link>
          </Button>
        </div>
      </div>

      <nav className="container flex flex-col items-center gap-4">
        <ul className="flex flex-wrap items-center justify-center gap-6">
          <li>
            <Link
              href="/"
              className="font-medium transition-opacity hover:opacity-75"
            >
              Tools
            </Link>
          </li>
          <li>
            <Link
              href="/pricing"
              className="font-medium transition-opacity hover:opacity-75"
            >
              Pricing
            </Link>
          </li>
          <li>
            <Link
              href="/about"
              className="font-medium transition-opacity hover:opacity-75"
            >
              About
            </Link>
          </li>
          <li>
            <Link
              href="/changelog"
              className="font-medium transition-opacity hover:opacity-75"
            >
              Changelog
            </Link>
          </li>
          <li>
            <a
              href="https://github.com/Develonaut/bnto"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-0.5 font-medium transition-opacity hover:opacity-75"
            >
              GitHub <ArrowUpRightIcon className="size-4" />
            </a>
          </li>
          <li>
            <a
              href="https://twitter.com/bntodev"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-0.5 font-medium transition-opacity hover:opacity-75"
            >
              Twitter <ArrowUpRightIcon className="size-4" />
            </a>
          </li>
        </ul>
        <ul className="flex flex-wrap items-center justify-center gap-6">
          <li>
            <Link
              href="/privacy"
              className="text-muted-foreground text-sm transition-opacity hover:opacity-75"
            >
              Privacy Policy
            </Link>
          </li>
        </ul>
      </nav>

      <div className="text-primary mt-10 w-full text-center md:mt-14 lg:mt-20">
        <Logo className="text-[20vw] leading-none tracking-tighter opacity-20 md:text-[15vw]" />
      </div>
    </footer>
  );
}
