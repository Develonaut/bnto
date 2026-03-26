import { Button, GithubIcon, Row } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/copy";
import { NewRecipeMobileButton } from "../NewRecipeMobileButton";
import { PAGE_LINKS } from "../nav";

interface MobileNavActionsProps {
  onClose: () => void;
}

function GitHubButton() {
  return (
    <Button
      variant="secondary"
      size="icon"
      href={GITHUB_URL}
      target="_blank"
      rel="noopener noreferrer"
    >
      <GithubIcon />
      <span className="sr-only">GitHub</span>
    </Button>
  );
}

export function MobileNavActions({ onClose }: MobileNavActionsProps) {
  return (
    <Row className="gap-4">
      <NewRecipeMobileButton onClick={onClose} />
      <Button
        variant="outline"
        href="/my-recipes"
        onClick={onClose}
        data-testid="mobile-link-my-recipes"
      >
        My Recipes
      </Button>
      {PAGE_LINKS.map((link) => (
        <Button
          key={link.href}
          variant="outline"
          href={link.href}
          onClick={onClose}
          data-testid={`mobile-link-${link.href.replace("/", "")}`}
        >
          {link.label}
        </Button>
      ))}
      <GitHubButton />
    </Row>
  );
}
