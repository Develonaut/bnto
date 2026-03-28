"use client";

import {
  Divider,
  FileUpload,
  FileUploadDropzone,
  Stepper,
  StepperContent,
  StepperList,
  StepperStep,
} from "@bnto/ui";
import { DropzoneContent } from "../DropzoneContent";
import { RunHeader } from "./RunHeader";
import { RunFileList } from "./RunFileList";
import { useRunTab } from "./useRunTab";

const noop = () => {};

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
        onRun={props.handleRun}
        onClear={props.handleClear}
        onDownloadAll={props.handleDownloadAll}
      />
      <Divider />
      <div className="min-h-0 flex-1 overflow-y-auto">
        <RunFileList
          phase={props.phase}
          inputFiles={props.inputFiles}
          results={props.results}
          errors={props.errors}
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
        <Stepper value={tab.step} onValueChange={noop} className="flex h-full flex-col">
          <StepperList>
            <StepperStep value="dropzone" label="Drop" />
            <StepperStep value="staging" label="Files" />
            <StepperStep value="running" label="Running" />
            <StepperStep value="completed" label="Done" />
            <StepperStep value="failed" label="Failed" />
          </StepperList>
          <StepperContent value="dropzone">
            <RunTabDropzone acceptLabel={tab.acceptLabel} />
          </StepperContent>
          <StepperContent value="staging">
            <RunTabContent {...tab} />
          </StepperContent>
          <StepperContent value="running">
            <RunTabContent {...tab} />
          </StepperContent>
          <StepperContent value="completed">
            <RunTabContent {...tab} />
          </StepperContent>
          <StepperContent value="failed">
            <RunTabContent {...tab} />
          </StepperContent>
        </Stepper>
      </FileUpload>
    </div>
  );
}

export { RunTab };
