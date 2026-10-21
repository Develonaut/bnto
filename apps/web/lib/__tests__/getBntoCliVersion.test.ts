import { describe, expect, it } from "vitest";
import { getBntoCliVersion } from "../getBntoCliVersion";

describe("getBntoCliVersion", () => {
  it("reads the version cargo install bnto installs", () => {
    expect(getBntoCliVersion()).toMatch(/^\d+\.\d+\.\d+/);
  });
});
