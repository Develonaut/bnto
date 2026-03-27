import { ArrowLeftIcon, Button, Row } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";

/** Back to Home + Report Issue action buttons for the 404 page. */
export function NotFoundActions() {
  return (
    <Row gap="sm" className="pt-4">
      <Button href="/" className="gap-2">
        <ArrowLeftIcon className="size-4" />
        Back to Home
      </Button>
      <Button
        variant="outline"
        href={`${GITHUB_URL}/issues`}
        target="_blank"
        rel="noopener noreferrer"
      >
        Report Issue
      </Button>
    </Row>
  );
}
