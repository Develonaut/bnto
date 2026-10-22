/** FAQ answers for CLI-only recipe pages, in place of the browser ones. */

import type { QA } from "./recipePageContent";

export const CLI_RECIPE_FAQ: QA[] = [
  {
    question: "Are my files private?",
    answer:
      "Yes. The recipe runs on your own machine through the bnto CLI, and nothing you process is uploaded. The CLI sends anonymous usage data, never file names, paths or contents. Turn it off with bnto telemetry disable.",
  },
  {
    question: "Can I use this on my phone?",
    answer:
      "No. This recipe needs the bnto CLI on a computer, because it drives tools from the computer's own operating system. Browser recipes on bnto.io work on any device.",
  },
];
