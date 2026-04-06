import { CopyButton, Row, TerminalIcon } from "@bnto/ui";

const INSTALL_COMMAND = "cargo install bnto";

/** Hero install row — copyable CLI install command, full width. */
export function HeroInstall() {
  return (
    <Row className="bg-muted/60 border-border gap-2 rounded-lg border px-4 py-2.5">
      <TerminalIcon className="text-muted-foreground size-4 shrink-0" />
      <code className="font-mono text-sm tracking-tight">{INSTALL_COMMAND}</code>
      <div className="ml-auto">
        <CopyButton value={INSTALL_COMMAND} label="Copy install command" />
      </div>
    </Row>
  );
}
