/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

import { assertProcessor } from "./assert";
import { editFieldsProcessor } from "./editFields";
import { fileCollectProcessor } from "./fileCollect";
import { fileCopyProcessor } from "./fileCopy";
import { fileDecodeProcessor } from "./fileDecode";
import { fileDeleteProcessor } from "./fileDelete";
import { fileFilterProcessor } from "./fileFilter";
import { fileJoinProcessor } from "./fileJoin";
import { fileListProcessor } from "./fileList";
import { fileMetadataProcessor } from "./fileMetadata";
import { fileMoveProcessor } from "./fileMove";
import { fileRenameProcessor } from "./fileRename";
import { fileRenderProcessor } from "./fileRender";
import { fileSortProcessor } from "./fileSort";
import { imageCompressProcessor } from "./imageCompress";
import { imageConvertProcessor } from "./imageConvert";
import { imageOverlayProcessor } from "./imageOverlay";
import { imageResizeProcessor } from "./imageResize";
import { imageStripExifProcessor } from "./imageStripExif";
import { reportProcessor } from "./report";
import { shellCommandProcessor } from "./shellCommand";
import { spreadsheetCleanProcessor } from "./spreadsheetClean";
import { spreadsheetConvertProcessor } from "./spreadsheetConvert";
import { spreadsheetExportProcessor } from "./spreadsheetExport";
import { spreadsheetLookupProcessor } from "./spreadsheetLookup";
import { spreadsheetMergeProcessor } from "./spreadsheetMerge";
import { spreadsheetReadProcessor } from "./spreadsheetRead";
import { spreadsheetRenameProcessor } from "./spreadsheetRename";
import { summarizeProcessor } from "./summarize";
import { vectorOptimizeProcessor } from "./vectorOptimize";
import { vectorRasterizeProcessor } from "./vectorRasterize";

export const PROCESSORS: readonly ProcessorDef[] = [
  assertProcessor,
  editFieldsProcessor,
  fileCollectProcessor,
  fileCopyProcessor,
  fileDecodeProcessor,
  fileDeleteProcessor,
  fileFilterProcessor,
  fileJoinProcessor,
  fileListProcessor,
  fileMetadataProcessor,
  fileMoveProcessor,
  fileRenameProcessor,
  fileRenderProcessor,
  fileSortProcessor,
  imageCompressProcessor,
  imageConvertProcessor,
  imageOverlayProcessor,
  imageResizeProcessor,
  imageStripExifProcessor,
  reportProcessor,
  shellCommandProcessor,
  spreadsheetCleanProcessor,
  spreadsheetConvertProcessor,
  spreadsheetExportProcessor,
  spreadsheetLookupProcessor,
  spreadsheetMergeProcessor,
  spreadsheetReadProcessor,
  spreadsheetRenameProcessor,
  summarizeProcessor,
  vectorOptimizeProcessor,
  vectorRasterizeProcessor,
] as const;
