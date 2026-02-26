export const ROOT_NAME = "FileUpload";
export const DROPZONE_NAME = "FileUploadDropzone";
export const TRIGGER_NAME = "FileUploadTrigger";
export const LIST_NAME = "FileUploadList";
export const ITEM_NAME = "FileUploadItem";
export const ITEM_PREVIEW_NAME = "FileUploadItemPreview";
export const ITEM_METADATA_NAME = "FileUploadItemMetadata";
export const ITEM_PROGRESS_NAME = "FileUploadItemProgress";
export const ITEM_DELETE_NAME = "FileUploadItemDelete";
export const CLEAR_NAME = "FileUploadClear";

export type Direction = "ltr" | "rtl";

export interface FileState {
  file: File;
  progress: number;
  error?: string;
  status: "idle" | "uploading" | "error" | "success";
}

export interface StoreState {
  files: Map<File, FileState>;
  dragOver: boolean;
  invalid: boolean;
}

export type StoreAction =
  | { type: "ADD_FILES"; files: File[] }
  | { type: "SET_FILES"; files: File[] }
  | { type: "SET_PROGRESS"; file: File; progress: number }
  | { type: "SET_SUCCESS"; file: File }
  | { type: "SET_ERROR"; file: File; error: string }
  | { type: "REMOVE_FILE"; file: File }
  | { type: "SET_DRAG_OVER"; dragOver: boolean }
  | { type: "SET_INVALID"; invalid: boolean }
  | { type: "CLEAR" };

export type Store = {
  getState: () => StoreState;
  dispatch: (action: StoreAction) => void;
  subscribe: (listener: () => void) => () => void;
};
