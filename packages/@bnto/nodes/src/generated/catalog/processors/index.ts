/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

import { fileCollectProcessor } from "./fileCollect";
import { fileCopyProcessor } from "./fileCopy";
import { fileFilterProcessor } from "./fileFilter";
import { fileMetadataProcessor } from "./fileMetadata";
import { fileRenameProcessor } from "./fileRename";
import { imageCompressProcessor } from "./imageCompress";
import { imageConvertProcessor } from "./imageConvert";
import { imageOverlayProcessor } from "./imageOverlay";
import { imageResizeProcessor } from "./imageResize";
import { imageStripExifProcessor } from "./imageStripExif";
import { shellCommandProcessor } from "./shellCommand";
import { spreadsheetCleanProcessor } from "./spreadsheetClean";
import { spreadsheetConvertProcessor } from "./spreadsheetConvert";
import { spreadsheetMergeProcessor } from "./spreadsheetMerge";
import { spreadsheetRenameProcessor } from "./spreadsheetRename";
import { vectorOptimizeProcessor } from "./vectorOptimize";
import { vectorRasterizeProcessor } from "./vectorRasterize";

export const PROCESSORS: readonly ProcessorDef[] = [
  fileCollectProcessor,
  fileCopyProcessor,
  fileFilterProcessor,
  fileMetadataProcessor,
  fileRenameProcessor,
  imageCompressProcessor,
  imageConvertProcessor,
  imageOverlayProcessor,
  imageResizeProcessor,
  imageStripExifProcessor,
  shellCommandProcessor,
  spreadsheetCleanProcessor,
  spreadsheetConvertProcessor,
  spreadsheetMergeProcessor,
  spreadsheetRenameProcessor,
  vectorOptimizeProcessor,
  vectorRasterizeProcessor,
] as const;
