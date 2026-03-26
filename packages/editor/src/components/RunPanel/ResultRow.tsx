import {
  Badge,
  Button,
  CheckCircle2Icon,
  DownloadIcon,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListMeta,
  FileListName,
  IconBadge,
} from "@bnto/ui";

function ResultRowIcon() {
  return (
    <FileListIcon>
      <IconBadge variant="primary" size="lg" aria-hidden="true">
        <CheckCircle2Icon className="size-5" />
      </IconBadge>
    </FileListIcon>
  );
}

function ResultRowName({ filename, extension }: { filename: string; extension?: string | null }) {
  return (
    <span className="flex items-center gap-1.5">
      <FileListName>{filename}</FileListName>
      {extension && (
        <Badge variant="outline" size="sm" className="shrink-0 uppercase">
          {extension}
        </Badge>
      )}
    </span>
  );
}

function ResultRowMeta({
  outputSize,
  originalSize,
  savings,
}: {
  outputSize: string;
  originalSize?: string;
  savings?: string;
}) {
  if (originalSize != null && savings != null) {
    return (
      <FileListMeta>
        <span className="line-through">{originalSize}</span>{" "}
        <span className="font-semibold text-primary">{savings}</span> {outputSize}
      </FileListMeta>
    );
  }
  return <FileListMeta>{outputSize}</FileListMeta>;
}

function ResultRowDownload({ filename, onDownload }: { filename: string; onDownload: () => void }) {
  return (
    <FileListActions>
      <Button
        variant="outline"
        size="icon"
        elevation="sm"
        onClick={onDownload}
        aria-label={`Download ${filename}`}
        data-testid="download-button"
      >
        <DownloadIcon className="size-4" />
      </Button>
    </FileListActions>
  );
}

function ResultRow({
  filename,
  extension,
  outputSize,
  originalSize,
  savings,
  onDownload,
}: {
  filename: string;
  extension?: string | null;
  outputSize: string;
  originalSize?: string;
  savings?: string;
  onDownload: () => void;
}) {
  return (
    <FileListItem data-testid="output-file">
      <ResultRowIcon />
      <FileListContent>
        <ResultRowName filename={filename} extension={extension} />
        <ResultRowMeta outputSize={outputSize} originalSize={originalSize} savings={savings} />
      </FileListContent>
      <ResultRowDownload filename={filename} onDownload={onDownload} />
    </FileListItem>
  );
}

export { ResultRow };
