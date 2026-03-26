"use client";

import { Divider, FileUpload, FileUploadDropzone } from "@bnto/ui";
import { DropzoneContent } from "../DropzoneContent";
import { RunHeader } from "./RunHeader";
import { RunFileList } from "./RunFileList";
import { useRunTab } from "./useRunTab";

function RunTabDropzone({ acceptLabel }: { acceptLabel: string }) {
  return (
    <div className="min-h-0 flex-1 p-3">
      <FileUploadDropzone disableAnimation className="h-full gap-3 p-6">
        <DropzoneContent label={acceptLabel} />
      </FileUploadDropzone>
    </div>
  );
}

function RunTabContent(props: ReturnType<typeof useRunTab>) {
  return (
    <>
      <RunHeader
        phase={props.phase}
        inputFiles={props.inputFiles}
        results={props.results}
        fileProgress={props.fileProgress}
        errors={props.errors}
        onBack={props.handleBack}
        onClear={props.handleClear}
        onDownloadAll={props.handleDownloadAll}
      />
      <Divider />
      <div className="min-h-0 flex-1 overflow-y-auto">
        <RunFileList
          phase={props.phase}
          inputFiles={props.inputFiles}
          results={props.results}
          fileProgress={props.fileProgress}
          onRemove={props.removeFile}
          onDownload={props.handleDownload}
        />
      </div>
    </>
  );
}

function RunTab() {
  const tab = useRunTab();
  return (
    <div className="flex h-full flex-col overflow-hidden rounded-[inherit] bg-input">
      <FileUpload
        value={tab.inputFiles}
        onValueChange={tab.setFiles}
        accept={tab.dropzoneAccept}
        multiple
        className="flex h-full flex-col gap-0"
      >
        {tab.showDropzone ? (
          <RunTabDropzone acceptLabel={tab.acceptLabel} />
        ) : (
          <RunTabContent {...tab} />
        )}
      </FileUpload>
    </div>
  );
}

export { RunTab };
