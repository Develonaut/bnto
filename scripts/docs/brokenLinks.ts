/**
 * Relative Markdown links that don't resolve to a file (ISS-20).
 *
 * A link resolves from the folder of the file it's in, as GitHub resolves it,
 * and a `/`-leading one from the repo root. One exception: `.claude/CLAUDE.md`
 * writes all its links from the repo root (`.claude/rules/…`), where agents
 * read it, so its links resolve from the root. Only the file is checked, not
 * the `#anchor`.
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

// `[text](target)` or `![alt](target)`, with an optional `"title"`.
const LINK = /\[[^\]]*\]\(([^)\s]+)(?:\s+"[^"]*")?\)/g;
const FENCE = /^\s*(`{3,}|~{3,})/;
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

/** Each line outside fenced code blocks, with code spans removed, and its 1-based number. */
function proseLines(source: string): [number, string][] {
  let fence: string | null = null;
  const lines: [number, string][] = [];
  source.split("\n").forEach((text, i) => {
    const marker = FENCE.exec(text)?.[1];
    if (marker && !fence) fence = marker[0];
    else if (marker && marker[0] === fence) fence = null;
    else if (!fence) lines.push([i + 1, text.replace(CODE_SPAN, "")]);
  });
  return lines;
}

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
    [...text.matchAll(LINK)]
      .map((match) => match[1])
      .filter((target) => isLocal(target) && !exists(resolveTarget(file, target)))
      .map((target) => ({ file, line, target })),
  );
}
