import { ArrowUpRightIcon, Button, CoffeeIcon, GithubIcon, Stack } from "@bnto/ui";
import { BUYMEACOFFEE_URL, GITHUB_URL } from "@/lib/links";

/** GitHub + Buy Me a Coffee links in the footer brand column. */
export function FooterBrandLinks() {
  return (
    <Stack gap="lg">
      <a
        href={GITHUB_URL}
        target="_blank"
        rel="noopener noreferrer"
        className="flex items-center gap-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
        aria-label="Open source — GitHub repository"
      >
        <GithubIcon className="size-4" />
        Open source
        <ArrowUpRightIcon className="size-3" />
      </a>
      <Button
        href={BUYMEACOFFEE_URL}
        target="_blank"
        rel="noopener noreferrer"
        variant="outline"
        className="w-fit"
        aria-label="Support bnto — Buy Me a Coffee"
      >
        <CoffeeIcon className="size-4" />
        Buy me a coffee
      </Button>
    </Stack>
  );
}
