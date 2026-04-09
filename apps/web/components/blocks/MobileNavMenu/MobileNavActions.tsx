import { Button, Row } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";
import { NewRecipeMobileButton } from "../NewRecipeMobileButton";

interface MobileNavActionsProps {
  onClose: () => void;
}

export function MobileNavActions({ onClose }: MobileNavActionsProps) {
  return (
    <Row className="gap-4">
      <Button variant="outline" href="/explore" onClick={onClose} data-testid="mobile-link-explore">
        Explore
      </Button>
      <NewRecipeMobileButton onClick={onClose} />
      <Button
        variant="primary"
        href={GITHUB_URL}
        target="_blank"
        rel="noopener noreferrer"
        onClick={onClose}
      >
        Get started
      </Button>
    </Row>
  );
}
