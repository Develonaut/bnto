/**
 * Relative Markdown links that don't resolve to a file (ISS-20).
 *
 * A link resolves from the folder of the file it's in, as GitHub resolves it,
 * and a `/`-leading one from the repo root. One exception: `.claude/CLAUDE.md`
 * writes all its links from the repo root (`.claude/rules/…`), where agents
 * read it, so its links resolve from the root. Only the file is checked, not
 * the `#anchor`.
 *
 * Checked: inline links and images, and reference definitions (`[ref]: path`).
 * Not checked: `<angle-bracket>` targets and HTML `href`/`src`. Today the
 * only such link is README.md's logo `<img src>`, which resolves.
 */

import { posix } from "node:path";

/** A link whose target doesn't exist. */
export interface BrokenLink {
  /** Repo-relative path of the file the link is in. */
  file: string;
  /** 1-based line number. */
  line: number;
  /** The target as written. */
  target: string;
}

/** The one file whose links resolve from the repo root. */
const ROOT_RELATIVE_FILE = ".claude/CLAUDE.md";

// `[text](target)` or `![alt](target)`, with an optional `"title"` or `'title'`.
const INLINE_LINK = /\[[^\]]*\]\(([^)\s]+)(?:\s+(?:"[^"]*"|'[^']*'))?\)/g;
// `[ref]: target`, at the start of a line. A `[^note]:` footnote isn't a link.
const REFERENCE = /^ {0,3}\[(?!\^)[^\]]+\]:\s*([^\s<]\S*)/g;
const FENCE = /^\s*(`{3,}|~{3,})(.*)$/;
const CODE_SPAN = /`[^`]*`/g;
const URL_SCHEME = /^[a-z][a-z0-9+.-]*:/i;

const isLocal = (target: string) => !URL_SCHEME.test(target) && !target.startsWith("#");

function decode(path: string): string {
  try {
    return decodeURIComponent(path);
  } catch {
    return path;
  }
}

/** Repo-relative path a local link target points at. */
function resolveTarget(file: string, target: string): string {
  const path = decode(target.split("#")[0]);
  if (path.startsWith("/")) return posix.normalize(path.slice(1));
  if (file === ROOT_RELATIVE_FILE) return posix.normalize(path);
  return posix.join(posix.dirname(file), path);
}

/**
 * The fence open after `text`, given the one `open` before it, or `undefined`
 * if `text` doesn't open or close one. As in CommonMark, only a bare fence of
 * the same character, at least as long as the opening one, closes a block.
 */
function nextFence(open: string | null, text: string): string | null | undefined {
  const match = FENCE.exec(text);
  if (!match) return undefined;
  const [, marker, rest] = match;
  if (!open) return marker;
  const closes = marker[0] === open[0] && marker.length >= open.length && !rest.trim();
  return closes ? null : undefined;
}

/** Each line outside fenced code blocks, with code spans removed, and its 1-based number. */
function proseLines(source: string): [number, string][] {
  let fence: string | null = null;
  const lines: [number, string][] = [];
  source.split("\n").forEach((text, i) => {
    const next = nextFence(fence, text);
    if (next !== undefined) fence = next;
    else if (!fence) lines.push([i + 1, text.replace(CODE_SPAN, "")]);
  });
  return lines;
}

const linkTargets = (text: string) =>
  [...text.matchAll(INLINE_LINK), ...text.matchAll(REFERENCE)].map((match) => match[1]);

/**
 * Links in `source` (the Markdown file at repo-relative `file`) whose target
 * doesn't exist. `exists` takes a repo-relative path.
 */
export function brokenLinks(
  file: string,
  source: string,
  exists: (path: string) => boolean,
): BrokenLink[] {
  return proseLines(source).flatMap(([line, text]) =>
    linkTargets(text)
      .filter((target) => isLocal(target) && !exists(resolveTarget(file, target)))
      .map((target) => ({ file, line, target })),
  );
}
