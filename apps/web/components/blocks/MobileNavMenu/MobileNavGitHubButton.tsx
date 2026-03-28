import { Button, GithubIcon } from "@bnto/ui";
import { GITHUB_URL } from "@/lib/links";

export function MobileNavGitHubButton() {
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
