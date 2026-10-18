import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

import {
  CURRENT_FORMAT_VERSION,
  SUPPORTED_FORMAT_VERSIONS,
  isSupportedVersion,
  isCompatibleVersion,
} from "./formatVersion";

describe("CURRENT_FORMAT_VERSION", () => {
  it("is a semver string", () => {
    expect(CURRENT_FORMAT_VERSION).toMatch(/^\d+\.\d+\.\d+$/);
  });

  it("is in the supported versions list", () => {
    expect(isSupportedVersion(CURRENT_FORMAT_VERSION)).toBe(true);
  });
});

describe("SUPPORTED_FORMAT_VERSIONS", () => {
  it("contains at least one version", () => {
    expect(SUPPORTED_FORMAT_VERSIONS.length).toBeGreaterThan(0);
  });
});

describe("isSupportedVersion", () => {
  it("returns true for the current version", () => {
    expect(isSupportedVersion("1.0.0")).toBe(true);
  });

  it("returns false for an unsupported version", () => {
    expect(isSupportedVersion("2.0.0")).toBe(false);
  });

  it("returns false for an empty string", () => {
    expect(isSupportedVersion("")).toBe(false);
  });

  it("returns false for a non-semver string", () => {
    expect(isSupportedVersion("latest")).toBe(false);
  });
});

describe("isCompatibleVersion", () => {
  it("returns true for exact match", () => {
    expect(isCompatibleVersion("1.0.0")).toBe(true);
  });

  it("returns true for same major, higher minor", () => {
    expect(isCompatibleVersion("1.3.0")).toBe(true);
  });

  it("returns true for same major, higher patch", () => {
    expect(isCompatibleVersion("1.0.5")).toBe(true);
  });

  it("returns false for different major version", () => {
    expect(isCompatibleVersion("2.0.0")).toBe(false);
  });

  it("returns false for major version 0", () => {
    expect(isCompatibleVersion("0.1.0")).toBe(false);
  });

  it("returns false for empty string", () => {
    expect(isCompatibleVersion("")).toBe(false);
  });

  it("returns false for non-semver string", () => {
    expect(isCompatibleVersion("latest")).toBe(false);
  });
});

describe("cross-language sync", () => {
  it("Rust FORMAT_VERSION matches TS CURRENT_FORMAT_VERSION", () => {
    // Resolve the monorepo root from this file, not via git: inside a git
    // hook GIT_DIR is set, and `git rev-parse --show-toplevel` in a worktree
    // then returns the current directory instead of the root.
    const root = resolve(dirname(fileURLToPath(import.meta.url)), "../../../..");
    const rustFile = resolve(root, "engine/crates/bnto-core/src/lib.rs");
    const source = readFileSync(rustFile, "utf-8");
    const match = source.match(/pub const FORMAT_VERSION:\s*&str\s*=\s*"([^"]+)"/);
    expect(match).not.toBeNull();
    expect(match![1]).toBe(CURRENT_FORMAT_VERSION);
  });
});
