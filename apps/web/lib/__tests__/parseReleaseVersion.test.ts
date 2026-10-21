import { describe, expect, it } from "vitest";
import { parseReleaseVersion } from "../parseReleaseVersion";

describe("parseReleaseVersion", () => {
  it("reads the [workspace.package] version", () => {
    const toml = '[workspace]\nmembers = []\n\n[workspace.package]\nversion = "1.2.3"\n';
    expect(parseReleaseVersion(toml)).toBe("1.2.3");
  });

  it("ignores dependency versions", () => {
    const toml =
      '[workspace.dependencies]\nserde = { version = "9.9.9" }\n\n[workspace.package]\nversion = "0.16.1"\n';
    expect(parseReleaseVersion(toml)).toBe("0.16.1");
  });

  it("throws when the workspace has no version", () => {
    expect(() => parseReleaseVersion('[workspace.package]\nrust-version = "1.88"\n')).toThrow(
      "version",
    );
  });
});
