import {
  buttonRowStyle,
  ghostButtonStyle,
  outlineButtonStyle,
  primaryButtonStyle,
} from "./globalErrorStyles";

interface GlobalErrorActionsProps {
  issueUrl: string;
  onReset: () => void;
}

/** Try Again / Report Issue / Back to Home buttons for the global error boundary. */
export function GlobalErrorActions({ issueUrl, onReset }: GlobalErrorActionsProps) {
  return (
    <div style={buttonRowStyle}>
      <button type="button" onClick={onReset} style={primaryButtonStyle}>
        Try Again
      </button>
      <a href={issueUrl} target="_blank" rel="noopener noreferrer" style={outlineButtonStyle}>
        Report Issue
      </a>
      {/* eslint-disable-next-line @next/next/no-html-link-for-pages -- global-error replaces the document, next/link router context unavailable */}
      <a href="/" style={ghostButtonStyle}>
        Back to Home
      </a>
    </div>
  );
}
