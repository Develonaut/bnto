import { Dropzone } from "@/components/ui/Dropzone";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { Upload } from "lucide-react";

export function DropzoneShowcase() {
  return (
    <Dropzone accept={{ "image/*": [] }}>
      {({ isDragActive }) => (
        <Stack gap="sm" align="center" className="text-muted-foreground">
          <Upload className="size-8" />
          <Text size="sm" weight="medium" color="inherit" as="span">
            {isDragActive ? "Drop files here" : "Drag images here or click to browse"}
          </Text>
          <Text size="xs" color="inherit" as="span">PNG, JPEG, WebP up to 10MB</Text>
        </Stack>
      )}
    </Dropzone>
  );
}
