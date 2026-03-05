/** Node category → compartment variant color mapping. */

import type { CompartmentVariant } from "./types";

const CATEGORY_VARIANT: Record<string, CompartmentVariant> = {
  image: "primary",
  spreadsheet: "secondary",
  file: "accent",
  data: "muted",
  network: "secondary",
  control: "warning",
  system: "muted",
  io: "info",
};

export { CATEGORY_VARIANT };
