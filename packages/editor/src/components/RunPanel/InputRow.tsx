import {
  Button,
  FileIcon,
  FileListActions,
  FileListContent,
  FileListIcon,
  FileListItem,
  FileListMeta,
  FileListName,
  IconBadge,
  LoaderIcon,
  XIcon,
  formatFileSize,
} from "@bnto/ui";

function InputRow({
  file,
  processing,
  onRemove,
}: {
  file: File;
  processing?: boolean;
  onRemove?: () => void;
}) {
  return (
    <FileListItem data-testid="input-file" aria-busy={processing}>
      <FileListIcon>
        <IconBadge variant="primary" size="lg" aria-hidden="true">
          {processing ? (
            <LoaderIcon className="size-5 motion-safe:animate-spin" />
          ) : (
            <FileIcon className="size-5" />
          )}
        </IconBadge>
      </FileListIcon>
      <FileListContent>
        <FileListName>{file.name}</FileListName>
        <FileListMeta>{formatFileSize(file.size)}</FileListMeta>
      </FileListContent>
      {onRemove && (
        <FileListActions>
          <Button
            variant="outline"
            size="icon"
            elevation="sm"
            onClick={onRemove}
            aria-label={`Remove ${file.name}`}
          >
            <XIcon className="size-4" />
          </Button>
        </FileListActions>
      )}
    </FileListItem>
  );
}

export { InputRow };
