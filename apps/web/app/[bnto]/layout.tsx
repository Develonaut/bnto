import Link from "next/link";
import { NavUserIsland } from "./_components/NavUserIsland";

/**
 * Bnto tool page layout -- SSR-safe with simplified header.
 *
 * Unlike the (app) layout which uses ssr: false for the entire shell,
 * this layout renders the header structure server-side for SEO.
 * Only NavUser is loaded client-side via NavUserIsland.
 */
export default function BntoLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex min-h-svh flex-col">
      <header className="sticky top-0 z-50 border-b bg-background">
        <div className="flex h-14 items-center gap-4 px-4 lg:px-6">
          <Link href="/" className="flex items-center gap-2">
            <span className="text-lg font-semibold">bnto</span>
          </Link>
          <div className="ml-auto flex items-center gap-2">
            <NavUserIsland />
          </div>
        </div>
      </header>
      <main className="flex flex-1 flex-col">{children}</main>
    </div>
  );
}
