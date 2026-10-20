# Workflow-Automation Platforms: Shell Execution with Workflow-Supplied Values, and How They Secure It

Research date: 2026-10-20. Sources are dated where possible. "Unverified" means no primary source was fetched for the claim.

## n8n Execute Command node: mechanism, interpolation, defaults, admin disable

### Takeaway

n8n's Execute Command node runs a free-form command string through the host's default shell. Since n8n 2.0 it is **disabled by default** via `NODES_EXCLUDE`, and n8n's own position is that the node is unsafe wherever workflow editors are not fully trusted. n8n has not fixed this as a vulnerability. Snyk tracks it as an unfixed High-severity "Command Injection via Execute Command node" issue, and the maintainers' answer is to disable the node, not to change its behaviour.

### Cited Findings

- **Mechanism (shell):** the node "runs shell commands on the host machine"; commands use the host's default shell (docs give `cmd` on Windows, `zsh` on macOS); in Docker it runs inside the n8n container; you can chain commands with `&&` or one per line, which only works because a shell parses the string — [n8n docs: Execute Command](https://docs.n8n.io/integrations/builtin/core-nodes/n8n-nodes-base.executecommand/)
- **Per-item execution:** with "Execute Once" off, the node runs once per input item, so each upstream item produces a separate shell invocation — [n8n docs: Execute Command](https://docs.n8n.io/integrations/builtin/core-nodes/n8n-nodes-base.executecommand/)
- **Security warning (verbatim):** "The Execute Command node can introduce significant security risks in environments that operate with untrusted users." — [n8n docs: Execute Command](https://docs.n8n.io/integrations/builtin/core-nodes/n8n-nodes-base.executecommand/)
- **Availability:** self-hosted only, in all editions; **not available on n8n Cloud** — [n8n docs: Execute Command](https://docs.n8n.io/integrations/builtin/core-nodes/n8n-nodes-base.executecommand/)
- **n8n 2.0 default-off:** the 2.0 breaking-changes page says n8n disables the ExecuteCommand and LocalFileTrigger nodes by default because they "pose security risks" and "allow users to run arbitrary commands and access the file system." To re-enable them, set `NODES_EXCLUDE="[]"` or remove those entries from the list — [n8n 2.0 breaking changes](https://docs.n8n.io/2-0-breaking-changes)
- **Admin disable syntax:** `NODES_EXCLUDE: "[\"n8n-nodes-base.executeCommand\", \"n8n-nodes-base.readWriteFile\"]"`. "Node type policies" are offered for blocking per project or without a restart — [n8n docs: Blocking nodes](https://docs.n8n.io/hosting/securing/blocking-nodes/)
- **Re-enable friction:** users reported that `N8N_NODES_INCLUDE` did not bring the node back. `NODES_EXCLUDE="[]"` worked, and Docker Compose users had to set it under the service's `environment:` key — [GitHub issue #23439](https://github.com/n8n-io/n8n/issues/23439); [n8n community thread](https://community.n8n.io/t/unable-to-re-enable-execute-command-node-in-n8n-2-0-using-documented-environment-variables/238232?tl=fr)
- **Snyk advisory SNYK-JS-N8N-13524400 (High, Command Injection via Execute Command node):** "An attacker can execute arbitrary commands on the host system by leveraging access to an authenticated user account... only exploitable if the Execute Command node is enabled and not all user accounts are strictly controlled and trusted." **No fixed version.** Snyk quotes the maintainers: no code change was made to the node's behaviour, and the recommended mitigation is to disable it by default where it is not required. The workaround is `NODES_EXCLUDE`. n8n.cloud is not affected — [Snyk n8n@2.9.1 listing](https://security.snyk.io/package/npm/n8n/2.9.1); [Snyk vuln page](https://security.snyk.io/vuln/SNYK-JS-N8N-13524400); [Snyk n8n@1.114.0](https://security.snyk.io/package/npm/n8n/1.114.0)
- **Other 2.0 hardening shipped alongside:** task runners on by default for all Code node executions; `N8N_BLOCK_ENV_ACCESS_IN_NODE` now defaults to `true`; ReadWriteFile and ReadBinaryFiles are restricted to `~/.n8n-files` by default (`N8N_RESTRICT_FILE_ACCESS_TO`); `$evaluateExpression()` is disabled in the Code node because "secure mode disables evaluating strings as code" (`N8N_RUNNERS_INSECURE_MODE=true` re-enables it but is "discouraged for production use") — [n8n 2.0 breaking changes](https://docs.n8n.io/2-0-breaking-changes)

### Inferences

- Values enter the command through n8n's general `{{ }}` expression system. The Command field is a normal parameter, so expressions resolve to text before the shell sees the string. That gives the same "template, then shell parses the result" problem as GitHub Actions `${{ }}` in `run:` (see the last section). The docs do not call this out.
- n8n's threat model treats "who can edit workflows" as the trust boundary. It does not treat "what data flows through workflows" as a boundary. The response was to switch the node off by default rather than make the node itself safe (for example, with argv mode or escaping).
- Blocking the node at the platform level, with admins opting in, is the closest peer analogue to bnto's consent gate. n8n gates per instance. bnto gates per recipe hash, which is finer-grained.

### Gaps

- No n8n documentation page was found that states how `{{ }}` expressions in the Command field are escaped or quoted. From training knowledge (unverified this session), the node uses Node's `child_process.exec` on the resolved string with no escaping. Verify against `packages/nodes-base/nodes/ExecuteCommand/ExecuteCommand.node.ts`.
- The exact n8n 2.0 GA release date was not confirmed from a primary source. Community threads about the node being "removed" appeared around the 2.0 release (late 2025, unverified): [community thread](https://community.n8n.io/t/execute-command-node-has-removed/233388?tl=ar).
- The Snyk advisory's publication date was not shown on the fetched page.
- The fetched pages did not confirm whether the Local File Trigger and SSH node identifiers are in the default 2.0 exclude list.

## n8n expression / Code-node sandbox CVEs and task-runner isolation

### Takeaway

n8n's expression evaluator and its Code-node sandboxes produced a run of Critical RCE advisories between December 2025 and mid-2026. Root causes were denylist-based sanitizers bypassed through JavaScript type coercion and in-process sandboxes escaped. n8n's structural fix is to run code out of process (task runners, preferably "external" mode in a sidecar container). Every advisory still lists "restrict who can edit workflows" as the real mitigation.

### Cited Findings

- **CVE-2025-68613 / GHSA-v98v-ff95-f3cp, "n8n Remote Code Execution via Expression Injection" (Critical), published 2025-12-19:** authenticated users can supply expressions during workflow configuration that "may run in an execution context that isn't adequately isolated from the underlying runtime," giving arbitrary code execution with the n8n process's privileges. CWE-913. Affects ≥0.211.0. Fixed in 1.120.4 / 1.121.1 / 1.122.0. Workarounds: restrict workflow create/edit to trusted users; harden the OS and network. These are described as short-term measures — [GitHub advisory](https://github.com/n8n-io/n8n/security/advisories/GHSA-v98v-ff95-f3cp); [GitLab advisory DB](https://advisories.gitlab.com/pkg/npm/n8n/CVE-2025-68613/)
  - A secondary source says CISA added it to the KEV catalog and cites Shadowserver counting more than 24,700 unpatched exposed instances in early February 2026. **Not verified against CISA directly.** Third parties give CVSS 9.9, also unverified against NVD — [Vulert](https://vulert.com/blog/?p=4628); [Ampcus Cyber](https://www.ampcuscyber.com/?p=11541)
- **CVE-2026-25049 / GHSA-6cqr-8cfr-67f8 (Critical), published 2026-02-04:** an expression-sanitizer bypass. `isSafeObjectProperty` checked property names against a denylist `Set` with `has()` (strict equality). The TypeScript `string` annotation is not enforced at runtime, so an array such as `["__proto__"]` passes the check and bracket notation then coerces it back to `__proto__`/`constructor`. The fix is a runtime type check. Fixed in 2.5.2 / 1.123.17. The write-up contradicts itself on whether authentication is required — [Endor Labs](https://www.endorlabs.com/learn/cve-2026-25049-n8n-rce)
- **CVE-2026-25115 / GHSA-8398-gmmx-564h (Python sandbox escape), published 2026-02-04:** fixed in 2.4.8. Severity drops when external runner mode is used, because the escape is confined to the sidecar — [GitHub Advisory DB](https://github.com/advisories/GHSA-8398-gmmx-564h); [Corgea](https://corgea.com/advisories/vulnerabilities/CVE-2026-25115)
- **CVE-2026-27494 / GHSA-mmgg-m5j7-f83h (Python Code node sandbox escape → file read/RCE), reported 9.9:** fixed in 1.123.22 / 2.9.3 / 2.10.1 — [GitLab advisory DB](https://advisories.gitlab.com/npm/n8n/CVE-2026-27494/); [safeguard.sh](https://safeguard.sh/resources/blog/cve-2026-27494-n8n-python-code-node-sandbox-escape)
- **CVE-2026-27495 / GHSA-jjpj-p2wh-qf23 (JavaScript Task Runner sandbox escape):** same fixed versions. CVSS is reported as 9.4 or 9.9 depending on the source. CISA SSVC lists exploitation as "None" — [exploit-intel](https://exploit-intel.com/vuln/CVE-2026-27495); [safeguard.sh](https://safeguard.sh/resources/blog/cve-2026-27495-n8n-javascript-task-runner-sandbox-escape)
- **CVE-2026-27496:** in external runner mode, impact is limited to data inside the external runner process. Fixed in 1.123.22 / 2.10.1 / 2.9.3 — [NVD](https://nvd.nist.gov/vuln/detail/CVE-2026-27496)
- **CVE-2026-49444 / GHSA-9pq8-m8gp-4p53 (Python Task Runner sandbox escape), dated 2026-06-16, 8.5 High:** fixed in 1.123.48 / 2.21.8 / 2.22.4 — [GitLab advisory DB](https://advisories.gitlab.com/npm/n8n/CVE-2026-49444/)
- **Scale:** the n8n GitHub security advisories list had 23 pages as of 2026-10-20. The latest batch (10 advisories, 2026-10-14) covers credential-permission, injection-into-filter (OData, Quick Base) and XSS issues, none about Execute Command or the Code node — [n8n security advisories](https://github.com/n8n-io/n8n/security/advisories)
- **Task runner isolation model:** without task runners, or in internal mode, "anyone who can edit a workflow could potentially read your database, encryption key, stored credentials, and environment variables." In internal mode the runner shares n8n's uid/gid. External mode runs a launcher in a sidecar container (`n8nio/runners` image) and is recommended for production. `N8N_RUNNERS_MODE` takes internal|external (default internal). `N8N_RUNNERS_AUTH_TOKEN` is required for external mode. `N8N_BLOCK_RUNNER_ENV_ACCESS` defaults to true for Python tasks — [n8n docs: Set up task runners](https://docs.n8n.io/deploy/host-n8n/configure-n8n/set-up-task-runners/); [n8n docs: task runner env vars](https://docs.n8n.io/hosting/configuration/environment-variables/task-runners/)
- **Python change in 2.0:** the Pyodide-based Python Code node was removed and replaced by native Python on task runners, which work only in external mode — [n8n 2.0 breaking changes](https://docs.n8n.io/2-0-breaking-changes)
- **Common workaround across these advisories:** upgrade, restrict workflow editing, use external runner mode, or add the Code node to `NODES_EXCLUDE` — [GitHub Advisory DB GHSA-8398-gmmx-564h](https://github.com/advisories/GHSA-8398-gmmx-564h)

### Inferences

- n8n's experience shows that **in-process language sandboxes plus denylist sanitizers fail repeatedly**: at least six RCE-class escapes in about six months. The durable control is process/container isolation plus limiting who can author workflows. For bnto, a "safe subset" that relies on denylisting interpreters (as the shell-command node does today) is the same class of control and should be treated as defence-in-depth, not a boundary.
- None of these CVEs involve the Execute Command node itself. n8n does not treat the node as vulnerable because executing commands is what it is for. Its risk is handled by default-off configuration rather than by CVEs.

### Gaps

- n8n's historical use of vm2 (vm2 was deprecated after multiple escapes in 2023) and when n8n moved off it could not be confirmed with a primary source this session.
- CVE-2025-68613 KEV status and CVSS were not verified against CISA or NVD.

## Windmill, Kestra, Prefect, Temporal, Dagster: how shell steps receive inputs

### Takeaway

The platforms split into two models. **Windmill** binds inputs to bash **positional parameters** (`$1`, `$2`, …), which is close to bnto's proposed `sh -c <literal> name <args...>`. **Kestra** renders Pebble `{{ inputs.x }}` **directly into command text**, and its own docs admit this "breaks outright when a value contains a quote or a newline." Kestra offers a context file and env vars as alternatives. **Prefect** and **Dagster** are Python libraries where the author builds the command string and can pass `env`. Windmill's one injection CVE came from interpolating values into generated source text, which is the anti-pattern.

### Cited Findings

**Windmill**

- Bash script inputs are inferred from `$1`, `$2`, `$3`; the documented pattern is `X="$1"` parsed as a string parameter, with `dflt="${2:-default value}"` for defaults. The last line of stdout, or `./result.json`, is the result — [Windmill docs: Bash quickstart](https://windmill.dev/docs/getting_started/scripts_quickstart/bash); [Windmill blog: bash script arguments](https://www.windmill.dev/blog/bash-scripts-arguments)
- Arguments are always strings. The guide says to omit the `#!/bin/bash` shebang. Windmill env vars such as `WM_WORKSPACE` and `WM_JOB_ID` are exposed — [Windmill write-script-bash skill (via playbooks.com)](https://playbooks.com/skills/windmill-labs/windmill/write-script-bash)
- A forum report says `${10}` and above are not recognised as inputs in the UI — [Windmill questions forum](https://questions.windmill.dev/m/1359414288220950628)
- Isolation: jobs run in an nsjail sandbox (`# sandbox <image>` runs a Docker image "inside the job's own nsjail sandbox, daemonless") — [Windmill docs: Bash quickstart](https://windmill.dev/docs/getting_started/scripts_quickstart/bash)
- **CVE-2026-33881 / GHSA-8q8j-mm3g-5c2q (Low), published 2026-03-25:** the NativeTS executor built a JS preamble by inserting workspace env-var values into single-quoted string literals **without escaping**. A value containing `'` breaks out and runs as code in every NativeTS script in the workspace, as the executing user (token capture, file reads). CWE-94. Only workspace admins can exploit it. Fixed in 1.664.0 — [GitHub advisory](https://github.com/windmill-labs/windmill/security/advisories/GHSA-8q8j-mm3g-5c2q)
- **CVE-2026-47107:** the nsjail config bind-mounted `/etc` read-write, so authenticated users could persistently poison `/etc/hosts`, `/etc/resolv.conf` and the CA bundle on the worker and intercept other users' `WM_TOKEN`s. Fixed in 1.703.2. CVSS is reported as 8.6 (INCIBE) or 9.6 (CraftedSignal) — [INCIBE](https://www.incibe.es/index.php/en/incibe-cert/early-warning/vulnerabilities/cve-2026-47107); [CraftedSignal brief](https://feed.craftedsignal.io/briefs/2026-05-windmill-nsjail-perms/); [OSV](https://osv.dev/vulnerability/CVE-2026-47107)

**Kestra**

- The Shell `Commands` task examples interpolate Pebble directly into command strings, e.g. `cat "{{ inputs.text_file }}"`, `cat {{ outputs.http_download.uri }}`. No escaping or injection warning is documented. Task runners are Process (host OS process APIs on the worker) or Docker (Docker volume mounts are "disabled by default for security reasons") — [Kestra plugin docs: Shell Commands](https://kestra.io/plugins/plugin-script-shell/io.kestra.plugin.scripts.shell.commands)
- Kestra's own reason for reading context from a file instead of inline Pebble: inline expressions are not valid source (lint failures), and "it breaks outright when a value contains a quote or a newline... It also mixes orchestration into business logic." Opt in with `executionContext: true`, which writes `.kestra-execution-context.json` and is off by default. `envs`/`globals` "already reach your script as environment variables" — [Kestra docs: Execution context](https://kestra.io/docs/scripts/execution-context)
- Kestra exposes `ENV_`-prefixed environment variables by default. A blueprint shows passing `secret()` values through task `env`, masked in logs — [Kestra docs: Variables](https://kestra.io/docs/workflow-components/variables); [Kestra blueprint](https://kestra.io/blueprints/environment-aware-secret-fetch)

**Prefect**

- `prefect-shell`'s `ShellOperation` block takes `commands` (a list of strings), `env`, `working_dir`, `shell` and `extension`. The official example passes values via `env={"today": today}`. There is also a `shell_run_command(command, env, cwd, return_all)` task — [Prefect docs: prefect-shell](https://docs.prefect.io/integrations/prefect-shell); [API ref](https://docs.prefect.io/integrations/prefect-shell/api-ref/prefect_shell-commands)

**Dagster**

- `create_shell_command_op` and `create_shell_script_op` in `dagster-shell` are deprecated (removal in 0.25) in favour of `PipesSubprocessClient` — [Dagster API docs: dagster-shell](https://docs.dagster.io/_apidocs/libraries/dagster-shell); [Dagster integration page](https://dagster.io/integrations/dagster-shell)
- A community answer on the Dagster forum warns that commands "built dynamically or include user input" open the door to command injection — [Dagster discuss](https://discuss.dagster.io/t/16595793/what-is-the-best-way-to-execute-a-system-bash-command-within)

**Temporal**

- No source found. Temporal has no built-in shell step. Activities are ordinary SDK code, so shell use is whatever the developer writes (e.g. Python `subprocess`). This is unverified this session.

### Inferences

- Windmill's bash model already ships "literal script authored by the workflow author; runtime values as positional parameters," the shape bnto is considering. It shows the pattern works in production. The fetched docs do not confirm whether Windmill passes values as real argv to bash or rewrites the script.
- Kestra's admission that inline templating "breaks outright" on quotes or newlines is a vendor-confirmed example of why bnto should never template the script body. Kestra's suggested fixes (context file, env vars) match the GitHub Actions guidance.
- Windmill CVE-2026-33881 is a direct precedent: "interpolate a value into generated source" turned into code injection even in an otherwise sandboxed product. That supports bnto's rule that the `sh -c` script is never templated.
- Prefect, Dagster and Temporal are code-first: the author writes Python, so command construction is the author's responsibility. They are not useful precedents for a declarative JSON recipe format. Kestra (YAML) and n8n/Huginn (UI/JSON) are the right comparisons.

### Gaps

- Windmill's exact bash invocation (whether it is `bash script.sh "$@"` with argv) was not confirmed. The "Security and process isolation" page was not fetched.
- Kestra's default `taskRunner` for Shell tasks and whether Commands wraps in `/bin/sh -c` by default were not confirmed. The page was truncated at 100k characters.
- No Kestra, Prefect, Dagster or Temporal CVE about shell injection was found. Not searched exhaustively.

## Zapier and Make: do they offer shell execution at all?

### Takeaway

Neither offers shell or command execution. Both offer only **hosted, resource-capped JavaScript/Python code steps** in an isolated sandbox. As multi-tenant SaaS, their threat model assumes the customer's code is hostile to the platform and to other tenants, so arbitrary host commands are not offered at all.

### Cited Findings

- Code by Zapier runs custom JavaScript or Python as Zap steps. Runtime limits are 1 s (Free), 30 s (Professional/Team) and 2 min (Enterprise), with 512 MB memory. Rate limits are 10 req/60 s on Free and 225 req/10 s on paid plans. Action steps can install npm/PyPI packages and use the Zapier SDK. Code _triggers_ cannot map fields from previous steps — [Zapier Help: Using Code by Zapier](https://help.zapier.com/hc/en-us/articles/45405528551181)
- Make's Code app (paid plans) runs JavaScript or Python in an environment Make describes as fully isolated: 1 CPU, 512 MB memory, built-in libraries (JS: moment, lodash; Python: pendulum, toolz, requests), with custom imports only on Enterprise — [Make help: The Make Code app is available](https://help.make.com/the-make-code-app-is-available.md); [Make blog](https://www.make.com/blog/make-code-app)

### Inferences

- Shell access exists only in **self-hosted, single-tenant** tools (n8n self-hosted only and not on n8n Cloud, Huginn, Windmill, Kestra), and even there it is increasingly off by default. bnto's CLI-first, local-execution model sits with the self-hosted tools, where the main risk is untrusted _recipe authors_ or untrusted _data_ rather than other tenants.

### Gaps

- No Zapier or Make page explicitly saying "no shell access" was found. The conclusion comes from the absence of any such feature in their docs. Network and file-system limits of Make's sandbox were not documented on the fetched pages.

## Huginn ShellCommandAgent

### Takeaway

Huginn's ShellCommandAgent Liquid-interpolates options, including the command itself (e.g. `{{cmd}}` from an event), and accepts either a string, which Ruby may hand to `/bin/sh`, or an argv array, which runs with no shell. It does nothing unless the operator sets `ENABLE_INSECURE_AGENTS=true`, which defaults to false. It has the same two layers bnto has: an argv mode and an operator opt-in gate.

### Cited Findings

- **Gate:** `should_run?` returns `ENV['ENABLE_INSECURE_AGENTS'] == "true"`. If it is false the agent logs "Unable to run because insecure agents are not enabled." The description warns: "This type of Agent runs arbitrary commands on your system... Only enable this Agent if you trust everyone using your Huginn installation." — [Huginn source: shell_command_agent.rb](https://github.com/huginn/huginn/blob/master/app/models/agents/shell_command_agent.rb)
- `.env.example` ships `ENABLE_INSECURE_AGENTS=false`: "Enable this setting to allow insecure Agents like the ShellCommandAgent. Only do this when you trust everyone using your Huginn installation." — [Huginn .env.example (CodeClimate mirror)](https://codeclimate.com/github/cantino/huginn/.env.example/source)
- **Execution:** `pid = spawn(*command, chdir: path, out: wout, err: werr, in: rin)`. `command` may be a String or an Array of Strings. Options are Liquid-interpolated per event (`handle(interpolated(event), event)`), and the docs say the command "could be defined as `{{cmd}}`" to take it from the event. There is no command allowlist or escaping — [Huginn source](https://github.com/huginn/huginn/blob/master/app/models/agents/shell_command_agent.rb)
- **Ruby semantics:** a single-string `spawn` goes to `/bin/sh` if it begins with a shell reserved word or special built-in, or contains meta characters. With `exe, *args`, "Ruby invokes the executable directly, with no shell and no shell expansion" — [Ruby 3.3 Process docs](https://docs.ruby-lang.org/en/3.3/Process.html)

### Inferences

- With a string command, a Liquid-interpolated event value containing `;` or `$(...)` triggers Ruby's shell path, so Huginn is open to injection from event data. The array form avoids that. Huginn documents both but does not steer users to the array. bnto's argv-only default is stricter than Huginn's.
- Huginn's gate trusts "everyone using the installation" (authors) and says nothing about untrusted _event data_, a gap peers share.

### Gaps

- No Huginn CVE or advisory about ShellCommandAgent was found (not searched exhaustively).

## Real-world template/expression → shell or code injection incidents and guidance

### Takeaway

The best-documented class of template-to-shell injection is GitHub Actions `${{ }}` inside `run:`. GitHub's official fix is exactly bnto's proposed design: never template the script; pass values out-of-band (an env var, or an action input) and reference them as quoted shell variables. Among workflow platforms, the confirmed incidents are expression/sandbox RCEs (n8n) and template-into-source injection (Windmill CVE-2026-33881). No CVE was found for a _data-driven_ shell injection through n8n's Execute Command node, because n8n treats that as intended behaviour for trusted editors.

### Cited Findings

- GitHub: `${{ }}` expressions "are evaluated and then substituted with the resulting values" before the `run` script executes. A PR title like `a"; ls $GITHUB_WORKSPACE"` breaks out of `title="${{ github.event.pull_request.title }}"` and runs commands — [GitHub Docs: Script injections](https://docs.github.com/en/actions/concepts/security/script-injections)
- GitHub's mitigations, in order: (1) pass the value to an action as an input; (2) assign it to an intermediate env var (`env: TITLE: ${{ github.event.pull_request.title }}`) and use `"$TITLE"` in the script, because the value "is held in memory as a variable" and is not "used to generate the script"; (3) double-quote shell variables to avoid word splitting — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
- Kestra concedes inline Pebble in scripts "breaks outright when a value contains a quote or a newline" — [Kestra docs: Execution context](https://kestra.io/docs/scripts/execution-context)
- Windmill CVE-2026-33881: an unescaped value interpolated into a generated JS string literal leads to code execution in other users' scripts — [GitHub advisory GHSA-8q8j-mm3g-5c2q](https://github.com/windmill-labs/windmill/security/advisories/GHSA-8q8j-mm3g-5c2q)
- n8n CVE-2025-68613 and CVE-2026-25049: the expression language is itself an RCE vector when its sandbox or sanitizer fails — [GHSA-v98v-ff95-f3cp](https://github.com/n8n-io/n8n/security/advisories/GHSA-v98v-ff95-f3cp); [Endor Labs](https://www.endorlabs.com/learn/cve-2026-25049-n8n-rce)
- n8n also published injection advisories where workflow values reach query languages unescaped, e.g. "OData $filter Injection in Microsoft Outlook and SharePoint v1 Nodes" and "Quick Base Node Allows Filter Injection Through the Where Parameter" (both 2026-10-14). These are the same class of bug in non-shell sinks — [n8n security advisories](https://github.com/n8n-io/n8n/security/advisories)

### Inferences

- bnto's proposed `sh -c '<literal>' name "$@"` with values only as positional args is structurally the same as GitHub's env-var mitigation. Data never enters the parsed script text, so shell metacharacters in values are inert. Residual risks, all of which GitHub, Kestra and Windmill also leave to the author:
  - the script itself misusing `$1`: unquoted expansion, `eval "$1"`, or passing `$1` to another interpreter or `sh -c`;
  - argument-option confusion: a value starting with `-` passed to a command, which needs `--` separators;
  - `name` (`$0`) must also be literal.
- Peers converge on three controls: (a) an opt-in gate at the operator/admin level (n8n 2.0 `NODES_EXCLUDE` default, Huginn `ENABLE_INSECURE_AGENTS`); (b) out-of-band value passing (GitHub env, Kestra context file/env, Windmill positional args); (c) process or container isolation (n8n external runners, Windmill nsjail). bnto has (a) per recipe hash and would gain (b) with the proposal. It has no (c), which is acceptable for a local CLI where the user is the operator, but worth noting.
- The interpreter denylist (sh/bash blocked, while `python3 -c`, `node -e` and `osascript -e` are allowed) is inconsistent with peer practice. No peer treats some interpreters as safer than others; they gate the capability as a whole. Allowing a _literal-only_ `sh -c` is arguably safer than today's allowed `python3 -c <templated>` if templating into `-c` code arguments is currently permitted.

### Gaps

- No public incident report was found of a production breach caused by data-driven shell injection through n8n Execute Command, Kestra Pebble-in-shell, or Huginn ShellCommandAgent specifically.
- Real-world GitHub Actions script-injection incidents (e.g. specific compromised repos) were not researched; only GitHub's official guidance was cited.
