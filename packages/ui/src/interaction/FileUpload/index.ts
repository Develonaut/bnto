import { FileUploadRoot, type FileUploadProps } from "./FileUploadRoot";
import { FileUploadDropzone } from "./FileUploadDropzone";
import { FileUploadList } from "./FileUploadList";
import { FileUploadItem } from "./FileUploadItem";
import { FileUploadItemMetadata } from "./FileUploadItemMetadata";
import { FileUploadItemActions } from "./FileUploadItemActions";
import { FileUploadItemDelete } from "./FileUploadItemDelete";
import { FileUploadClear } from "./FileUploadClear";

const FileUpload = Object.assign(FileUploadRoot, {
  Root: FileUploadRoot,
  Dropzone: FileUploadDropzone,
  List: FileUploadList,
  Item: FileUploadItem,
  ItemMetadata: FileUploadItemMetadata,
  ItemActions: FileUploadItemActions,
  ItemDelete: FileUploadItemDelete,
  Clear: FileUploadClear,
});

export { FileUpload };
export type { FileUploadProps };
