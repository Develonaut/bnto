# Dogfood recipes

Repo chores that bnto does for itself. Each one runs from the repo root with the CLI built from this checkout (`task cli:build`, which every task below runs first).

| Recipe                                   | Task                   | What it does                                                                                                                                                        |
| ---------------------------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [size-budget](size-budget.bnto.json)     | `task check:budgets`   | Fails naming every file in `apps/web/public` over its budget: 6 MB for the WASM bundle, 512 KB for anything else.                                                   |
| [grab-bags](grab-bags.bnto.json)         | `task check:grab-bags` | Fails naming every `utils`, `helpers` or `hooks` `.ts`/`.tsx` file. Checks the files after `--`, else the staged ones (minus `e2e/`).                               |
| [unused-assets](unused-assets.bnto.json) | `task check:assets`    | Fails naming every file in `apps/web/public` that nothing in the web app or its UI packages references. Skips the WASM bundle, favicons, `robots.txt` and dotfiles. |
| [favicons](favicons.bnto.json)           | `task assets:favicons` | Renders the four PNG favicons in `apps/web/public/favicon/` from `favicon.svg`.                                                                                     |

`task check:recipes` runs `bnto check recipes/`. CI's engine job runs that check, and its `web-assets` job runs the size budget and the unused-asset check on every web or engine change, so a node change that breaks a recipe, or a web change that adds an unreferenced asset, fails the PR.

The pre-commit grab-bag hook stays the shell script in `scripts/hooks/check-grab-bags.sh`: a hook can't count on contributors having a built `bnto` on their PATH.
