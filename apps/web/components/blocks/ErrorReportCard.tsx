import { Card, CardContent, CardFooter } from "@bnto/ui";

import { ErrorReportContent } from "./ErrorReportContent";
import { ErrorReportActions } from "./ErrorReportActions";

interface ErrorReportCardProps {
  message: string;
  issueUrl: string;
  onReset: () => void;
}

export function ErrorReportCard({ message, issueUrl, onReset }: ErrorReportCardProps) {
  return (
    <Card className="max-w-lg w-full px-8 py-10">
      <CardContent>
        <ErrorReportContent message={message} />
      </CardContent>
      <CardFooter>
        <ErrorReportActions issueUrl={issueUrl} onReset={onReset} />
      </CardFooter>
    </Card>
  );
}
