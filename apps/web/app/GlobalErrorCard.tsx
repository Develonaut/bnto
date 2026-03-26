import { GlobalErrorIcon } from "./GlobalErrorIcon";
import { GlobalErrorActions } from "./GlobalErrorActions";
import { cardStyle, errorMessageStyle, headingStyle, textStyle } from "./globalErrorStyles";

interface GlobalErrorCardProps {
  message: string;
  issueUrl: string;
  onReset: () => void;
}

/** Error card content — icon, heading, message, and action buttons. */
export function GlobalErrorCard({ message, issueUrl, onReset }: GlobalErrorCardProps) {
  return (
    <div style={cardStyle}>
      <GlobalErrorIcon />
      <h1 style={headingStyle}>Something went wrong</h1>
      <p style={textStyle}>
        An unexpected error occurred. You can try again or report this issue so we can fix it.
      </p>
      <p style={errorMessageStyle}>{message}</p>
      <GlobalErrorActions issueUrl={issueUrl} onReset={onReset} />
    </div>
  );
}
