import Link from "next/link";
import { Stack } from "@bnto/ui";
import { RECIPES } from "../nav";

interface MobileNavExploreProps {
  onClose: () => void;
}

export function MobileNavExplore({ onClose }: MobileNavExploreProps) {
  return (
    <Stack className="gap-10">
      <div className="text-2xl font-display font-bold text-primary-foreground">Explore</div>
      <div className="grid w-full grid-cols-2 gap-x-4 gap-y-10">
        {RECIPES.map((category) => (
          <Stack key={category.title} className="gap-4 text-primary-foreground">
            <div className="text-xs uppercase tracking-wider text-primary-foreground/60">
              {category.title}
            </div>
            <Stack as="ul" className="gap-3">
              {category.links.map((link) => (
                <li key={link.url}>
                  <Link
                    href={link.url}
                    onClick={onClose}
                    data-testid={`mobile-link-${link.url.replace("/", "")}`}
                    className="text-lg leading-normal font-medium text-primary-foreground"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </Stack>
          </Stack>
        ))}
      </div>
    </Stack>
  );
}
