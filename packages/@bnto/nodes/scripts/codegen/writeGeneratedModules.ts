import type { Writer } from "./createWriter";
import { engineSchemaEntriesFile } from "./fields/engineSchemaEntriesFile";
import { enumConstantsFile } from "./fields/enumConstantsFile";
import { paramFieldInfoFile } from "./fields/paramFieldInfoFile";
import { convexLabelsFile } from "./files/convexLabelsFile";
import { definitionSchemaFile } from "./files/definitionSchemaFile";
import { formatVersionFile } from "./files/formatVersionFile";
import { i18nStringsFile } from "./files/i18nStringsFile";
import {
  CONVEX_LABELS_OUTPUT,
  DEF_SCHEMA_OUTPUT,
  ENGINE_SCHEMA_ENTRIES_OUTPUT,
  ENUM_CONSTANTS_OUTPUT,
  FORMAT_VERSION_OUTPUT,
  I18N_STRINGS_OUTPUT,
  PARAM_FIELD_INFO_OUTPUT,
} from "./paths";
import type { CatalogEnvelope } from "./types";

/** The single-file outputs: field configs, enum lists, schema, version, Convex labels, i18n. */
export function writeGeneratedModules(out: Writer, catalog: CatalogEnvelope): void {
  out.write(ENGINE_SCHEMA_ENTRIES_OUTPUT, engineSchemaEntriesFile(catalog));
  out.write(PARAM_FIELD_INFO_OUTPUT, paramFieldInfoFile(catalog));
  out.write(ENUM_CONSTANTS_OUTPUT, enumConstantsFile(catalog));
  out.write(DEF_SCHEMA_OUTPUT, definitionSchemaFile(catalog));
  out.write(FORMAT_VERSION_OUTPUT, formatVersionFile(catalog.version));
  out.write(CONVEX_LABELS_OUTPUT, convexLabelsFile(catalog));
  out.write(I18N_STRINGS_OUTPUT, i18nStringsFile(catalog));
}
