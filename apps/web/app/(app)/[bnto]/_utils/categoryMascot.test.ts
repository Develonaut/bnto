import { describe, expect, it } from "vitest";
import { getCategoryMascot } from "./categoryMascot";

describe("getCategoryMascot", () => {
  it("returns sushi-thumbsup for image", () => {
    expect(getCategoryMascot("image")).toBe("/mascots/sushi-thumbsup.svg");
  });

  it("returns octopus-chef for spreadsheet", () => {
    expect(getCategoryMascot("spreadsheet")).toBe("/mascots/octopus-chef.svg");
  });

  it("returns sumo-sushi for file", () => {
    expect(getCategoryMascot("file")).toBe("/mascots/sumo-sushi.svg");
  });

  it("returns penguin-chef for vector", () => {
    expect(getCategoryMascot("vector")).toBe("/mascots/penguin-chef.svg");
  });

  it("returns sumo-sushi fallback for unknown category", () => {
    expect(getCategoryMascot("unknown")).toBe("/mascots/sumo-sushi.svg");
  });
});
