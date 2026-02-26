/**
 * Site-wide messaging constants.
 *
 * Single source of truth for how we describe bnto to users and crawlers.
 * Every consumer of marketing copy (layout metadata, footer, gallery,
 * JSON-LD, llms.txt) should reference or match these strings.
 *
 * When updating copy here, also update `public/llms.txt` (static file,
 * can't import JS).
 */

/** Site title shown in browser tab and SERP. */
export const SITE_TITLE = "bnto";

/** Default page title with value prop. */
export const DEFAULT_TITLE = "bnto - Compress, Clean & Convert Files Free";

/** Title template for child pages. `%s` is replaced by the page title. */
export const TITLE_TEMPLATE = "%s - bnto";

/** One-line description for meta tags, OG, and Twitter cards. */
export const SITE_DESCRIPTION =
  "Compress images, clean CSVs, rename files, and convert formats. Free, instant, 100% in your browser. No signup, no upload. Open source.";

/** Short tagline for compact spaces (footer, badges). */
export const TAGLINE = "Compress images, clean CSVs, rename files, and convert formats.";

/** Privacy and trust message. */
export const TRUST_LINE = "Free, instant, private. Processed in your browser.";

/** License footer line. */
export const LICENSE_LINE = "MIT Licensed. Browser tools free forever.";

/** Gallery heading. */
export const GALLERY_HEADING = "Pick a tool. Drop your files.";

/** Gallery subheading. */
export const GALLERY_SUBHEADING =
  "Compress images, clean CSVs, rename files, and convert formats. All in your browser. No upload limits, no account required.";

/** GitHub repo URL. */
export const GITHUB_URL = "https://github.com/Develonaut/bnto";
