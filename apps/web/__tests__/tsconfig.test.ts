import { copyFileSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { writeConfigurationDefaults } from "next/dist/lib/typescript/writeConfigurationDefaults";
import ts from "typescript";
import { afterEach, describe, expect, it, vi } from "vitest";

/**
 * The isolated E2E server (`E2E_PORT`) runs `next dev` with
 * `NEXT_DIST_DIR=.next-e2e` and writes route types there. A run stopped
 * mid-write leaves them truncated, and `next build` type-checks every file
 * `tsconfig.json` includes, so those types must stay out of it (ISS-49), even
 * after the E2E server has set up TypeScript the way Next does on start.
 */

const WEB = resolve(__dirname, "..");
const E2E_DIST_DIR = ".next-e2e";
const E2E_TYPES = [".next-e2e/dev/types/routes.d.ts", ".next-e2e/types/routes.d.ts"];
const TRUNCATED = "type Routes = {\n  href: ";

async function tsconfigPathWith(distDir?: string) {
  vi.resetModules();
  vi.stubEnv("NEXT_DIST_DIR", distDir);
  const { default: config } = await import("../next.config");
  return config.typescript?.tsconfigPath;
}

/** A copy of the app's tsconfigs beside a page and truncated E2E route types. */
function makeApp(tsconfigs: string[]) {
  const dir = mkdtempSync(join(tmpdir(), "web-tsconfig-"));
  for (const name of tsconfigs) copyFileSync(join(WEB, name), join(dir, name));
  writeFileSync(join(dir, "page.ts"), "export const page = 1;\n");
  for (const file of E2E_TYPES) {
    mkdirSync(dirname(join(dir, file)), { recursive: true });
    writeFileSync(join(dir, file), TRUNCATED);
  }
  return dir;
}

/** What the build's type-check sees: its files and their syntax errors. */
function typeCheckBuild(dir: string) {
  const host = { ...ts.sys, onUnRecoverableConfigFileDiagnostic: () => {} };
  const parsed = ts.getParsedCommandLineOfConfigFile(join(dir, "tsconfig.json"), {}, host);
  const program = ts.createProgram(parsed?.fileNames ?? [], parsed?.options ?? {});
  return {
    files: program.getRootFileNames().map((file) => file.slice(dir.length + 1)),
    errors: program.getSyntacticDiagnostics().map((d) => d.file?.fileName ?? "(global)"),
  };
}

afterEach(() => {
  vi.unstubAllEnvs();
});

describe("the build's tsconfig.json", () => {
  it("doesn't type-check the E2E server's route types after that server starts", async () => {
    const e2eTsconfig = (await tsconfigPathWith(E2E_DIST_DIR)) ?? "tsconfig.json";
    const dir = makeApp(["tsconfig.json", e2eTsconfig]);
    try {
      // What the E2E server's `next dev` does to the tsconfig it's given on start.
      await writeConfigurationDefaults(
        ts.version,
        join(dir, e2eTsconfig),
        false, // isFirstTimeSetup
        true, // hasAppDir
        E2E_DIST_DIR,
        false, // hasPagesDir
        true, // isolatedDevBuild (Next 16's default)
      );
      const { files, errors } = typeCheckBuild(dir);
      expect(files).toContain("page.ts");
      for (const file of E2E_TYPES) expect(files).not.toContain(file);
      expect(errors).toEqual([]);
    } finally {
      rmSync(dir, { recursive: true, force: true });
    }
  });

  it("is the tsconfig the build and `task dev` use", async () => {
    expect(await tsconfigPathWith()).toBeUndefined();
  });
});
