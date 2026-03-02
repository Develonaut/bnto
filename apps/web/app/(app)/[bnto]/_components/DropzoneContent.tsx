import { UploadIcon } from "@/components/ui/icons";

/**
 * Presentational content for the file dropzone.
 *
 * Renders the upload icon, "Drag & drop" prompt, and accepted-types label.
 * Used inside `<FileUpload.Dropzone>` by RecipeShell and showcase components.
 */
export function DropzoneContent({ label }: { label: string }) {
  return (
    <>
      <div className="rounded-full bg-muted p-3 text-muted-foreground">
        <UploadIcon className="size-6" />
      </div>
      <div className="text-center">
        <p className="text-sm font-medium text-foreground">
          Drag & drop files here
        </p>
        <p className="mt-1 text-xs text-muted-foreground">
          or click to browse &middot; accepts {label}
        </p>
      </div>
    </>
  );
}
