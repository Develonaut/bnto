/**
 * "How it works" steps for a CLI-only recipe: install, run, done locally.
 * Browser recipes use the drop-run-download flow in recipePageContent.ts.
 */

import type { Recipe } from "@bnto/core";
import { getCliRunCommand } from "@bnto/registry";
import type { Step } from "./recipePageContent";

export function getCliHowItWorks(recipe: Recipe): Step[] {
  return [
    {
      title: "Install the CLI",
      description: "Install bnto with cargo install bnto. One binary, no account, no signup.",
    },
    {
      title: "Run it from your terminal",
      description: `Run ${getCliRunCommand(recipe)}. bnto info ${recipe.slug} lists its settings and anything it needs installed first.`,
    },
    {
      title: "It runs on your machine",
      description:
        "The recipe runs natively with the tools your operating system provides. Nothing you process is uploaded to bnto.",
    },
  ];
}
