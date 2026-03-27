import { Button, Row } from "@bnto/ui";
import { NewRecipeMobileButton } from "../NewRecipeMobileButton";
import { PAGE_LINKS } from "../nav";
import { MobileNavGitHubButton } from "./MobileNavGitHubButton";

interface MobileNavActionsProps {
  onClose: () => void;
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
      <Button variant="outline" href="/explore" onClick={onClose} data-testid="mobile-link-explore">
        Explore
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
      <MobileNavGitHubButton />
    </Row>
  );
}
