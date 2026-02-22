import Link from "next/link";
import { Footer } from "../_components/Footer";
import { ThemeToggleIsland } from "./_components/ThemeToggleIsland";
import { NavUserIsland } from "./_components/NavUserIsland";

/**
 * Bnto tool page layout -- SSR-safe with floating glassmorphic pill header.
 *
 * Unlike the (app) layout which uses ssr: false for the entire shell,
 * this layout renders the header structure server-side for SEO.
 * Only NavUser and ThemeToggle are loaded client-side via islands.
 */
export default function BntoLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex min-h-svh flex-col">
      <header className="fixed top-4 left-1/2 z-50 w-[min(90%,700px)] -translate-x-1/2">
        <div className="flex h-14 items-center gap-4 rounded-[2rem] border border-border/50 bg-background/70 px-4 shadow-md backdrop-blur-md">
          <Link href="/" className="flex items-center gap-2">
            <span className="font-display text-lg font-semibold">bnto</span>
          </Link>
          <div className="ml-auto flex items-center gap-1">
            <ThemeToggleIsland />
            <NavUserIsland />
          </div>
        </div>
      </header>
      <main className="flex flex-1 flex-col pt-24">{children}</main>
      <Footer />
    </div>
  );
}
