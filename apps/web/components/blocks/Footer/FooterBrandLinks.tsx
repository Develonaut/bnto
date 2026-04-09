import { Beer } from "lucide-react";
import { Button, GithubIcon, Row } from "@bnto/ui";
import { BUYMEACOFFEE_URL, GITHUB_URL } from "@/lib/links";

/** GitHub + Buy Me a Beer icon buttons in the footer brand column. */
export function FooterBrandLinks() {
  return (
    <Row className="gap-2">
      <Button
        href={GITHUB_URL}
        target="_blank"
        rel="noopener noreferrer"
        variant="outline"
        size="icon"
        aria-label="GitHub repository"
      >
        <GithubIcon className="size-4" />
      </Button>
      <Button
        href={BUYMEACOFFEE_URL}
        target="_blank"
        rel="noopener noreferrer"
        variant="warning"
        size="icon"
        aria-label="Buy me a beer"
      >
        <Beer className="size-4" />
      </Button>
    </Row>
  );
}
