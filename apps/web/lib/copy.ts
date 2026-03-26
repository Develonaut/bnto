/**
 * Site-wide messaging constants.
 *
 * Single source of truth for how we describe bnto to users and crawlers.
 * Every consumer of marketing copy (layout metadata, footer, gallery,
 * JSON-LD, llms.txt) should reference or match these strings.
 *
 * The `/llms.txt` and `/llms-full.txt` routes auto-generate from `RECIPES`
 * in `@bnto/nodes` — no manual sync needed.
 */

/** Site title shown in browser tab and SERP. */
export const SITE_TITLE = "bnto";

/** Default page title with value prop. */
export const DEFAULT_TITLE = "bnto - Free Tools That Run in Your Browser";

/** Title template for child pages. `%s` is replaced by the page title. */
export const TITLE_TEMPLATE = "%s - bnto";

/** One-line description for meta tags, OG, and Twitter cards. */
export const SITE_DESCRIPTION =
  "Free tools that run in your browser — compress images, clean CSVs, rename files, convert formats, and build custom recipes. No signup, no upload. Open source.";

/** Short tagline for compact spaces (footer, badges). */
export const TAGLINE =
  "Free tools that run in your browser. Compress, clean, convert, rename — or build your own.";

/** Privacy and trust message. */
export const TRUST_LINE = "Free, instant, runs in your browser.";

/** License footer line. */
export const LICENSE_LINE = "MIT Licensed. Browser recipes free forever.";

/** Gallery heading. */
export const GALLERY_HEADING = "Pick a recipe. Drop your files.";

/** Gallery subheading. */
export const GALLERY_SUBHEADING =
  "Compress, clean, convert, and rename — or build your own recipes. Everything runs in your browser. No uploads, no limits, no account required.";

/** GitHub repo URL. */
export const GITHUB_URL = "https://github.com/Develonaut/bnto";

/** Buy Me a Coffee URL. Update username after creating account at buymeacoffee.com. */
export const BUYMEACOFFEE_URL = "https://buymeacoffee.com/develonaut";
