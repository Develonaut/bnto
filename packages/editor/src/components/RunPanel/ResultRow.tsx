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
  const hasSavings = originalSize != null && savings != null;
  return (
    <FileListItem data-testid="output-file">
      <FileListIcon>
        <IconBadge variant="primary" size="lg" aria-hidden="true">
          <CheckCircle2Icon className="size-5" />
        </IconBadge>
      </FileListIcon>
      <FileListContent>
        <span className="flex items-center gap-1.5">
          <FileListName>{filename}</FileListName>
          {extension && (
            <Badge variant="outline" size="sm" className="shrink-0 uppercase">
              {extension}
            </Badge>
          )}
        </span>
        <FileListMeta>
          {hasSavings ? (
            <>
              <span className="line-through">{originalSize}</span>{" "}
              <span className="font-semibold text-primary">{savings}</span> {outputSize}
            </>
          ) : (
            outputSize
          )}
        </FileListMeta>
      </FileListContent>
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
    </FileListItem>
  );
}

export { ResultRow };
