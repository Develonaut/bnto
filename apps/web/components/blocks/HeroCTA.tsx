import { GithubIcon } from "@bnto/ui";
import { Button, Row } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";

export function HeroCTA() {
  return (
    <Row className="gap-3 pt-2">
      <Button href="#faq" elevation="sm">
        Learn more
      </Button>
      <Button
        variant="outline"
        href={GITHUB_URL}
        target="_blank"
        rel="noopener noreferrer"
        elevation="sm"
      >
        <GithubIcon />
        GitHub
      </Button>
    </Row>
  );
}
