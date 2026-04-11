import { describe, expect, it } from "vitest";
import { getCategoryMascot } from "./categoryMascot";

describe("getCategoryMascot", () => {
  it("returns sushi-thumbsup for image", () => {
    expect(getCategoryMascot("image")).toBe("/mascots/sushi-thumbsup.svg");
  });

  it("returns sushi-onigiri for spreadsheet", () => {
    expect(getCategoryMascot("spreadsheet")).toBe("/mascots/sushi-onigiri.svg");
  });

  it("returns salmon-chopstick for file", () => {
    expect(getCategoryMascot("file")).toBe("/mascots/salmon-chopstick.svg");
  });

  it("returns penguin-chef for vector", () => {
    expect(getCategoryMascot("vector")).toBe("/mascots/penguin-chef.svg");
  });

  it("returns octopus-chef fallback for unknown category", () => {
    expect(getCategoryMascot("unknown")).toBe("/mascots/octopus-chef.svg");
  });
});
