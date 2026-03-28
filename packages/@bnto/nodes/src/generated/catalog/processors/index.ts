/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

import type { ProcessorDef } from "../types";

import { fileRenameProcessor } from "./fileRename";
import { imageCompressProcessor } from "./imageCompress";
import { imageConvertProcessor } from "./imageConvert";
import { imageResizeProcessor } from "./imageResize";
import { imageStripExifProcessor } from "./imageStripExif";
import { spreadsheetCleanProcessor } from "./spreadsheetClean";
import { spreadsheetConvertProcessor } from "./spreadsheetConvert";
import { spreadsheetMergeProcessor } from "./spreadsheetMerge";
import { spreadsheetRenameProcessor } from "./spreadsheetRename";

export const PROCESSORS: readonly ProcessorDef[] = [
  fileRenameProcessor,
  imageCompressProcessor,
  imageConvertProcessor,
  imageResizeProcessor,
  imageStripExifProcessor,
  spreadsheetCleanProcessor,
  spreadsheetConvertProcessor,
  spreadsheetMergeProcessor,
  spreadsheetRenameProcessor,
] as const;
