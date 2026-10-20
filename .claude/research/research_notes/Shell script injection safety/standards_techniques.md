# Shell Script Injection Safety: Standards and Low-Level Techniques (evaluating bnto's proposed `script` param)

> Method note: Shell startup and environment-import claims were checked against primary docs. They were also run on this machine (macOS, `/bin/bash` 3.2.57, `/bin/sh` → bash in sh mode, `/bin/dash`, `/bin/ksh` 93u+ 2012-08-01, `/bin/zsh` 5.9). Local runs are labeled **[local test]**, and each one sits next to the man-page or manual URL that documents the behavior. Linux distros ship newer bash (5.x), which may differ in small ways. Those spots are flagged.

## 1. OWASP layered defenses, CWE-78 / CWE-88, and `--`

### Takeaway

OWASP ranks the defenses: (1) avoid OS commands and shells entirely, (2) escaping is a weak fallback, (3) if a command is unavoidable, use structured argument separation **plus** allowlist validation of both the command and its arguments. Argument (option) injection is a separate threat that survives "no shell" execution. `--` helps only for programs that honor it, and it does not make a value safe in every sense.

### Cited Findings

- "The primary defense is to avoid calling OS commands directly." Built-in library functions are the preferred alternative. — [OWASP OS Command Injection Defense Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- Escaping (e.g. PHP `escapeshellarg()`) is a weaker option 2: even with it, "an attacker can still pass a single argument to the command". Escaping stops command chaining but not option injection. — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- Option 3, parameterization plus input validation. Layer 1: "use structured mechanisms that automatically enforce the separation between data and command". Layer 2: commands "must be validated against a list of allowed commands", and arguments go through an allowlist or an allowlist regex with a length limit (e.g. `^[a-z0-9]{3,10}$`) that excludes metacharacters and whitespace. — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- "Every OS Command Injection is also an Argument Injection." Values like `--help` change behavior even when metacharacters are blocked. OWASP: "For commands that support it, use `--` to end option parsing". It also warns that for curl "`--` does not prevent additional transfers" (use one validated URL plus `--globoff`), and that argument separation does not stop SSRF. — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- OWASP .NET guidance: do not pass untrusted input to `cmd.exe`, `powershell.exe`, or `sh -c`. `ArgumentList` "handles escaping, but untrusted input remains a security risk." — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- Least privilege is listed as an additional defense. — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- CWE-88 definition: the product "constructs a string for a command to be executed by a separate component … but it does not properly delimit the intended arguments, options, or switches". Mitigations: avoid building a single command string, use array-style argument APIs, and use "accept known good" validation: "Do not rely exclusively on looking for malicious or malformed inputs … However, denylists can be useful for detecting potential attacks". — [CWE-88](https://cwe.mitre.org/data/definitions/88.html)
- CWE-88 shows `--` only inside a demonstrative example ("ls -l --" so remaining args are treated as filenames). It does **not** discuss the limits of `--`. — [CWE-88](https://cwe.mitre.org/data/definitions/88.html)
- CWE-88 observed examples include CVE-2022-36069 (a dependency tool allowed leading-dash option injection into git, leading to code execution), CVE-2016-10033 (PHPMailer `-o`/`-X` sendmail switches leading to file write and RCE), CVE-2019-13475 (injected `-exec` leading to RCE), and CVE-2007-0882 (Solaris telnetd `-f` auth bypass). — [CWE-88](https://cwe.mitre.org/data/definitions/88.html)

### Inferences

- bnto's current `args`-array design is OWASP option 3, layer 1 (structured separation). It has no allowlist validation of argument _values_ and no argument-injection guard. A `{{item.name}}` that evaluates to `--output=/etc/x` or `-oProxyCommand=...` reaches the target binary as an option. This applies to both the existing `command`+`args` mode and the proposed `script` mode, wherever the script author writes `cmd "$1"` without `--`.
- `--` is a per-program convention (getopt-style). It cannot be enforced by bnto generically, because some programs ignore it and some (like curl) still act dangerously on a well-formed non-option value. The realistic engine-level control is documentation plus an optional per-field "no leading dash" validator. Prefixing file paths with `./` when relative is a common alternative (inference; not sourced here).

### Gaps

- I did not fetch the CWE-78 page itself. CWE-88's framing and the OWASP cheat sheet cover the same ground. OWASP's statement that every OS command injection is also argument injection ties the two together.

## 2. Is `sh -c 'script' name "$value"` (positional parameters) safe?

### Takeaway

Yes, it is the authoritative recommended pattern. POSIX defines `$0` and `$1…` from the operands after the `-c` string, and ShellCheck (SC2156) prescribes it over splicing values into the script text. Safety holds only if the _script_ treats the parameters as data: always double-quoted, never passed to `eval`, `sh -c` again, `source`, arithmetic contexts, or similar sinks, and with `--` for commands that take options.

### Cited Findings

- POSIX `sh -c`: "Read commands from the command_string operand", "Set the value of special parameter 0 … from the value of the command_name operand and the positional parameters ($1, $2, and so on) in sequence from the remaining argument operands." "No commands shall be read from the standard input." — [POSIX sh (IEEE Std 1003.1-2024)](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/sh.html)
- If `command_name` is omitted, `$0` is taken from the first argument. So the `<name>` slot must always be supplied, or the first data value shifts into `$0`. — [POSIX sh](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/sh.html)
- ShellCheck SC2156 flags `find … -exec sh -c 'i="{}"; …'` because "the filename is passed by injecting it into a shell string … This can break the script and allow arbitrary code execution exploits." The correct form is `sh -c 'i="$1"; sox "$i" …' shell {} \;`, where the filename "will be safely treated as literal text". It also advises against using the `$0` slot for data. — [ShellCheck SC2156](https://www.shellcheck.net/wiki/SC2156)
- **[local test]** `sh -c 'printf "[%s]\n" "$1"' bnto '$(echo INJECTED); rm -rf x \`id\`'`printed the value literally:`[$(echo INJECTED); rm -rf x \`id\`]`. No expansion occurred. (Behavior follows from POSIX operand semantics above.)
- **[local test] Caveat: unquoted `$1`.** `sh -c 'printf "[%s]" $1' bnto 'a * b'` (run in `/`) produced `[a][Applications][bin]…[b]`. The value was word-split and glob-expanded. This is not code execution, but it is file-list injection and argument injection.
- **[local test] Caveat: `eval`.** `sh -c 'eval "echo $1"' bnto 'x; echo EVAL_INJECTED'` printed `x` and then `EVAL_INJECTED`. Any re-parse sink in the script turns data back into code.
- Argument injection still applies inside the script. A quoted `"$1"` beginning with `-` is still an option to the command that receives it. — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html), [CWE-88](https://cwe.mitre.org/data/definitions/88.html)
- Industry precedent for the same "values out of the script text" principle: Apache Airflow warns that for `BashOperator` "escaping and sanitization of the Bash command is not performed". It tells users to "pass this via the `env` kwarg and use double-quotes inside the Bash command" (`bash_command="echo \"…'$message'\""`, `env={"message": '{{ dag_run.conf[...] }}'}`) instead of Jinja-templating values into `bash_command`. — [Airflow BashOperator docs](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)

### Inferences

- bnto's proposal (`<shell> -c <script> <name> <args...>`, script never templated, `{{` rejected in script) is the SC2156 / POSIX-correct pattern. Airflow's env-var variant is equivalent in safety. Positional params have the advantage of not colliding with or overriding real environment variables. Env vars have the advantage of named access. Both keep data out of the parser.
- The remaining injection risk sits entirely in script-author hygiene (unquoted expansions, `eval`, nested `sh -c "$1"`, `xargs`, `find -exec sh -c "...$1..."`, `[[ $1 -eq … ]]` / `$(( $1 ))` arithmetic, which in bash evaluates array subscripts with command substitution). The engine cannot enforce this. Running ShellCheck on scripts (e.g. at recipe validation or in `bnto doctor`/lint) would catch SC2086 (unquoted) and SC2156-style issues. Inference: ShellCheck is not a dependency today.
- Arithmetic-context injection is a known bash hazard, but I did not fetch a source for it in this session (see Gaps).

### Gaps

- I did not fetch Greg's Wiki BashFAQ/050 or BashFAQ/048 (eval), Unix StackExchange canonical answers, or ShellCheck SC2016. SC2156 plus POSIX cover the core claim. The bash arithmetic-subscript injection hazard (`$(( a[$(cmd)] ))`) is noted from general knowledge only, with no source fetched.

## 3. Rust `std::process::Command` on Windows: BatBadBut (CVE-2024-24576) and its follow-up

### Takeaway

On Windows, `CreateProcess` silently runs `.bat`/`.cmd` files through `cmd.exe`, whose parsing differs from the MSVCRT argv rules. Before Rust 1.77.2, `Command::arg` values passed to a batch file could inject commands. Rust's fix escapes what it can and returns `InvalidInput` otherwise. That fix was then bypassed with trailing spaces or dots in the filename (CVE-2024-43402, fixed in 1.81.0). Because bnto resolves bare names on PATH (PATHEXT includes `.CMD`/`.BAT`, e.g. npm shims), this is directly relevant. bnto needs Rust ≥ 1.81.0, and it should treat "spawn a .cmd/.bat with templated args" as risky even when patched.

### Cited Findings

- Rust advisory: the standard library "did not properly escape arguments when invoking batch files (with the `bat` and `cmd` extensions) on Windows". Attackers controlling arguments could bypass escaping and run arbitrary shell commands. All Rust versions before 1.77.2 on Windows were affected. "No other platform or use is affected." — [Rust blog, CVE-2024-24576](https://blog.rust-lang.org/2024/04/09/cve-2024-24576/)
- Mitigation: the team "didn't identify a solution that would correctly escape arguments in all cases". `Command` now returns an `InvalidInput` error at spawn time when it cannot safely escape. `CommandExt::raw_arg` bypasses std escaping entirely (for callers doing their own escaping or passing trusted input). — [Rust blog](https://blog.rust-lang.org/2024/04/09/cve-2024-24576/)
- Incomplete fix: "it was possible to bypass the fix when the batch file name had trailing whitespace or periods". Tracked as CVE-2024-43402. Affects all versions < 1.81.0, patched in 1.81.0. — [GHSA-2xg3-7mm6-98jj](https://github.com/rust-lang/rust/security/advisories/GHSA-2xg3-7mm6-98jj)
- Root cause per the discoverer: "`CreateProcess()` implicitly spawns `cmd.exe` when executing batch files", and cmd.exe has different escaping rules. Exploitable when the command has no extension or ends in `.bat`/`.cmd` (both in default `PATHEXT`), arguments are user-controlled, and the runtime doesn't escape properly for cmd.exe. — [Flatt Security, BatBadBut](https://flatt.tech/research/posts/batbadbut-you-cant-securely-execute-commands-on-windows/)
- Vendor responses: Haskell, Node.js, PHP, and Rust patched. Java "won't fix". Erlang, Go, Python, and Ruby updated documentation only. — [Flatt Security](https://flatt.tech/research/posts/batbadbut-you-cant-securely-execute-commands-on-windows/)
- Developer mitigations from the write-up: always specify the file extension (e.g. `test.exe`) so a batch file on PATH isn't picked up; if batch files must run, escape yourself. Caret-escaping quotes is insufficient because cmd expands `%VAR%` first; `%` needs special handling (`%%cd:~,%`), plus `/V:OFF` and `/E:ON`. The author calls this tricky and possibly not the best approach. — [Flatt Security](https://flatt.tech/research/posts/batbadbut-you-cant-securely-execute-commands-on-windows/)

### Inferences

- Residual risks for bnto on Windows. (a) The Rust advisory doesn't cover invoking `cmd.exe /c …` explicitly, where std applies normal MSVCRT quoting that cmd does not honor. The existing denylist blocks `cmd`/`cmd.exe` by name, but only by name. (b) An `InvalidInput` spawn error is fail-closed but surfaces as a confusing runtime failure. bnto should map it to a clear "argument can't be safely passed to a .cmd/.bat shim" message. (c) The fix only triggers when std _recognizes_ the target as a batch file, and the 43402 bypass shows that recognition is fragile. Pinning MSRV ≥ 1.81.0 (or newer) for Windows builds is required.
- Option: when PATH resolution lands on a `.bat`/`.cmd`, bnto could refuse templated args, or require that the recipe name the extension explicitly. This is consistent with Flatt's "specify the extension" advice.

### Gaps

- Not verified: whether Rust std's batch-file detection covers every PATHEXT resolution path bnto uses (e.g. if bnto resolves the full path itself via a `which`-style crate before calling `Command::new`, detection works on the resolved extension). This needs a code check in bnto.

## 4. PowerShell: `-Command`, `-File`, `-EncodedCommand`, `-CommandWithArgs`, env vars, injection guidance

### Takeaway

With `-Command <string>`, every following argument becomes part of the command text, so trailing data args are code. `-File` passes trailing args as literal strings to script parameters. `-EncodedCommand` only solves quoting of trusted text, not data separation. PowerShell 7.4+ (mainstream in 7.5) adds `-CommandWithArgs`, which populates `$args` like POSIX `sh -c`. Windows PowerShell 5.1 has no such switch, so env vars (`$env:X`) or a temp `.ps1` with `-File` are the data channels there.

### Cited Findings

- pwsh 7.x `-Command`: "If the value of **Command** is a string, **Command** must be the last parameter for pwsh, because all arguments following it are interpreted as part of the command to execute." A string passed to `-Command` "is still executed as PowerShell code". — [about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh)
- Windows PowerShell 5.1 has the same rule for `-Command` ("all arguments following it are interpreted as part of the command to execute"). — [about_PowerShell_exe (5.1)](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1)
- `-File`: "all characters typed after the File parameter name are interpreted as the script filepath followed by the script parameters". "Parameters passed to the script are passed as literal strings, after interpretation by the current shell". `$Env:windir` passed via `-File` from cmd.exe arrives as the literal string `$Env:windir`. As of 7.2, `-File` accepts only `.ps1` on Windows. Array-valued parameters aren't supported via `-File`. — [about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh)
- **`-CommandWithArgs` / `-cwa`** (experimental in 7.4, "mainstream in PowerShell 7.5-preview.5"): "Unlike `-Command`, this parameter populates the `$args` built-in variable". Example: `pwsh -CommandWithArgs '$args | % { "arg: $_" }' arg1 arg2`. — [about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh)
- The 5.1 syntax lists no `-CommandWithArgs`. It lists `-EncodedCommand` and `-EncodedArguments` ("Base64-encoded string version command arguments … UTF-16LE"). — [about_PowerShell_exe (5.1)](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1)
- `-EncodedCommand` takes Base64 UTF-16LE text, "Use this parameter to submit commands to PowerShell that require complex, nested quoting." — [about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh)
- `-NoProfile`: "Doesn't load the PowerShell profiles". So profiles load by default and must be suppressed for determinism. `-NonInteractive` turns prompts into errors. — [about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh)
- Microsoft (PowerShell team) on injection: "The most common form of mistake is script injection, where a script author takes a parameter value (supplied by an attacker) and runs it in a trusted context". The specific sink list lives in the InjectionHunter module and DEFCON talk, not in the post. — [PowerShell Injection Hunter blog](https://devblogs.microsoft.com/powershell/powershell-injection-hunter-security-auditing-for-powershell-scripts/)
- Airflow's env-var channel (see §2) is the same idea applied to bash. — [Airflow docs](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)

### Inferences

- bnto's proposed Windows design (env vars) is correct for 5.1 and portable to 7.x. For pwsh ≥ 7.5, `-CommandWithArgs` gives positional parity with POSIX (`$args[0]` ≈ `$1`) but leaves 5.1 unsupported. A single env-var scheme (`BNTO_ARG_1…N` / `BNTO_ARG_COUNT`, or named per param) works on both, so it's simpler to keep one channel. Recommended invocation: `powershell.exe -NoProfile -NonInteractive -EncodedCommand <b64(script)>` with data only in env. Encoding the _trusted_ script sidesteps Windows command-line quoting of the script text entirely, while data never touches the command line.
- `-File` with a temp `.ps1` is another option. However, its args still traverse the Windows command line ("after interpretation by the current shell"), so pwsh's own command-line parser re-splits them. Env vars avoid the command line entirely.
- Even with env vars, data becomes code again if the script uses `Invoke-Expression`, `[scriptblock]::Create`, `$ExecutionContext.InvokeCommand.ExpandString`, `Add-Type` with interpolated source, or passes `$env:X` to native executables. Before 7.3 (`$PSNativeCommandArgumentPassing`), native-arg quoting was buggy. These sinks come from general knowledge, since the Injection Hunter post did not enumerate them (see Gaps).
- Env names: Windows env names are case-insensitive, so a `BNTO_ARG_*` prefix avoids colliding with `PATH`, `PSModulePath`, etc. Strip or override `PSModulePath`: a hostile module path can shadow cmdlets via module autoloading. That is an inference, not sourced.

### Gaps

- I didn't fetch about_Parsing or about_Invoke-Expression, so the Invoke-Expression warning text and the `$PSNativeCommandArgumentPassing` behavior aren't directly cited. The InjectionHunter rule list (InjectionRisk.InvokeExpression, .AddType, .ExpandString, etc.) was not retrieved. The post links to the PowerShell Gallery module only.
- Whether `PSModulePath` / `__PSLockdownPolicy` env injection affects a `-NoProfile` child is not verified.

## 5. Denylisting interpreters vs allowlists vs trust boundaries

### Takeaway

A name-based shell denylist that still admits `python3 -c`, `perl -e`, `node -e`, `osascript -e`, `ruby -e`, `awk`, `find -exec`, `env`, `xargs`, `git -c core.sshCommand`, and so on does not prevent arbitrary code execution _by the recipe author_. Standards bodies say not to rely on denylists. What actually matters is who authors the code: the recipe/script author is trusted only through consent and review (pinned hash), and data flowing in must never become code. That data-vs-code separation is enforced structurally, not by name filtering.

### Cited Findings

- CWE-88: use "accept known good" validation; "Do not rely exclusively on looking for malicious or malformed inputs"; denylists are "useful for detecting potential attacks". — [CWE-88](https://cwe.mitre.org/data/definitions/88.html)
- OWASP: commands "must be validated against a list of allowed commands" (allowlist, not denylist). — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html)
- GTFOBins catalogs ordinary Unix binaries that can spawn shells or run arbitrary code (e.g. via `-e`/`-c`/`-exec`-style options). This shows how large the non-shell interpreter surface is. — [GTFOBins](https://gtfobins.github.io/)
- Home Assistant `shell_command` draws the boundary at authorship: "the command name itself cannot be generated by a template, but it must be literally provided". Templated commands run in "a more secure environment that doesn't allow any shell helpers" (no `~`, `|`, `>`/`>>`). Templates may only fill arguments. — [Home Assistant shell_command](https://www.home-assistant.io/integrations/shell_command/)
- Airflow draws the same boundary: the `bash_command` author is trusted, while `dag_run.conf` (user-submittable via the Web UI) is the untrusted input and must flow via `env`. — [Airflow docs](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)
- Budibase CVE-2026-25044 shows what happens when this boundary is missing: "the bash automation step executes user-provided commands using execSync … User input is processed through processStringSync which allows template interpolation". Fixed in 3.33.4. — [CVE-2026-25044 (CVE.org record)](https://cveawg.mitre.org/api/cve/CVE-2026-25044)

### Inferences

- The current denylist is security theater against a malicious recipe author: `python3 -c 'import os; os.system(...)'` already equals `sh -c`. Its only real value is a speed bump that nudges authors toward args-array use and keeps "shell" out of the simple path. With an explicit `script` mode, the denylist mostly creates confusion ("why is bash banned but python allowed?"). It should be reframed or retired in favor of: (a) the consent and hash pin as the author-trust control, and (b) the no-templating-in-code rule as the data-trust control.
- The rule that does matter is that templates never reach any code-bearing argument. Today `{{…}}` can be substituted into `args` of `python3 -c "<code>"` or `osascript -e`, which recreates the injection the `script` design forbids. For consistency, bnto should consider rejecting `{{` in the argument immediately following known code flags (`-c`, `-e`, `--eval`, `-Command`) of known interpreters. A simpler alternative is to document that interpreters must receive data as additional argv (`python3 -c 'import sys; print(sys.argv[1])' {{x}}`), which is safe because Python, Perl, and Node expose trailing args via `sys.argv`/`@ARGV`/`process.argv`. That is an inference; per-interpreter docs were not fetched.
- Consent pinned to a SHA-256 of the definition matches the author-trust model. Its strength depends on the hash covering everything that becomes code (script text, command, args templates, env keys, shell choice, `cwd`), and on re-prompting on any change.

### Gaps

- No single authoritative source states "the denylist is meaningless if `python -c` is allowed". The conclusion is inferred from CWE/OWASP allowlist guidance plus GTFOBins.

## 6. Environment-variable and startup-file hazards when invoking sh/bash/zsh/dash/ksh

### Takeaway

Before the script's first line runs, a shell can execute code from: `BASH_ENV` (bash non-interactive, _not_ when invoked as `sh`), exported functions `BASH_FUNC_name%%` (bash, including as `sh`), `SHELLOPTS=xtrace` + `PS4='$(cmd)'` (bash, including as `sh`), `BASHOPTS` (bash ≥ 4.1), `ENV` (interactive shells only), `/etc/zshenv` and `$ZDOTDIR/.zshenv` (**every** zsh, including `-c`, unless `-f`), and `FPATH` function autoload (ksh93). The robust control is to `env_clear()` and pass an allowlist, not to strip a denylist. Use `bash -p` and `zsh -f` as belt-and-braces.

### Cited Findings

- **bash `BASH_ENV`**: "When bash is started non-interactively, to run a shell script, for example, it looks for the variable BASH_ENV in the environment … and uses the expanded value as the name of a file to read and execute." It acts like `if [ -n "$BASH_ENV" ]; then . "$BASH_ENV"; fi`. — [GNU Bash manual: Bash Startup Files](https://www.gnu.org/software/bash/manual/html_node/Bash-Startup-Files.html) (text verified in local `man bash`)
  - **[local test]** `BASH_ENV=… /bin/bash -c 'echo ran'` → `BASH_ENV_SOURCED ran` (yes, `-c` counts as non-interactive). `/bin/bash --posix -c` → not sourced. `/bin/sh -c` (bash as sh) → not sourced.
- **bash as `sh`**: "a non-interactive shell invoked with the name sh does not attempt to read any other startup files". In posix mode, `ENV` is read only by interactive shells. — [GNU Bash manual: Bash Startup Files](https://www.gnu.org/software/bash/manual/html_node/Bash-Startup-Files.html)
- **POSIX `ENV`**: "when and only when an interactive shell is invoked". — [POSIX sh](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/sh.html)
  - **[local test]** `ENV=…` was not sourced by `bash -c`, `sh -c`, `dash -c`, or `ksh -c`. It _was_ sourced by `dash -ic`.
- **dash**: ENV is read "on entry to an interactive shell". — [dash(1) man page](https://man7.org/linux/man-pages/man1/dash.1.html) (verified against local `man dash`)
- **bash `-p` (privileged)**: "the $ENV and $BASH_ENV files are not processed, shell functions are not inherited from the environment, and the SHELLOPTS … variable[s], if [they appear] in the environment, [are] ignored". In bash 4+/5 the list also includes BASHOPTS, CDPATH, and GLOBIGNORE. — [GNU Bash manual: The Set Builtin](https://www.gnu.org/software/bash/manual/html_node/The-Set-Builtin.html) (local 3.2 man page lists ENV, BASH_ENV, functions, SHELLOPTS)
  - **[local test]** `BASH_ENV=… bash -p -c 'echo ran'` → `ran` only. `SHELLOPTS=xtrace PS4='$(…)' bash -p -c true` → no code executed.
- **`SHELLOPTS` / `BASHOPTS` / `PS4`**: if SHELLOPTS (bash) or BASHOPTS (bash ≥ 4.1) is in the environment at startup, the listed options are enabled before reading startup files. `PS4` is expanded and printed before each traced command under `xtrace`. — [GNU Bash manual: Bash Variables](https://www.gnu.org/software/bash/manual/html_node/Bash-Variables.html)
  - **[local test]** `env SHELLOPTS=xtrace PS4='$(echo PS4_CODE_EXEC >&2)' /bin/bash -c true` → prints `PS4_CODE_EXEC`: **code execution from two env vars, no file needed**. The same works for `/bin/sh -c` on macOS (bash-as-sh). It did not work for `/bin/dash`. On bash 3.2, `BASHOPTS` had no effect (expected, since it was introduced in 4.1).
- **Exported functions (Shellshock lineage)**: post-Shellshock bash imports functions from variables named `BASH_FUNC_<name>%%`.
  - **[local test]** `env 'BASH_FUNC_ls%%=() { echo HIJACKED_ls; }' /bin/bash -c 'ls /nonexist'` → `HIJACKED_ls`. Same result with `--posix` and with macOS `/bin/sh`. _Not_ imported by `dash` or `zsh`. Any command name in the script (`ls`, `cp`, `rm`) can be overridden by an attacker-controlled env var. `bash -p` disables this per the manual above.
- **IFS**: **[local test]** `env IFS=x bash -c` showed IFS reset to space/tab/newline. bash ignores an inherited IFS. (POSIX permits implementations to ignore IFS from the environment. That was not directly quoted in the fetched POSIX page, which mentions IFS only in rationale.) — [POSIX sh](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/sh.html)
- **zsh**: "Commands are first read from /etc/zshenv; this cannot be overridden. … Commands are then read from $ZDOTDIR/.zshenv. If the shell is a login shell … /etc/zprofile … Then, if the shell is interactive … /etc/zshrc …". `$ZDOTDIR`defaults to`$HOME`. RCS and GLOBAL_RCS options gate later files. — [zsh manual: Files / Startup files](https://zsh.sourceforge.io/Doc/Release/Files.html) (verified in local `man zshall`, "STARTUP/SHUTDOWN FILES")
  - **[local test]** `ZDOTDIR=<dir with .zshenv> /bin/zsh -c 'echo ran'` → `ZSHENV_SOURCED ran`. With `zsh -f -c` (NO_RCS) and `zsh --no-rcs -c` → not sourced. `/etc/zshenv` is absent on this macOS. If present, it is root-owned and "cannot be overridden".
- **ksh93 `FPATH`**: **[local test]** `FPATH=<dir containing file foo defining function foo> /bin/ksh -c foo` → `FPATH_AUTOLOAD`. ksh93 autoloads undefined commands as functions from `FPATH`, so an env-supplied FPATH can hijack command names. (Local test only. I did not fetch the ksh93 man page.)
- **macOS `/bin/sh`** is bash (3.2) running as `sh` on this machine. **[local test]** It honored `BASH_FUNC_*` and `SHELLOPTS`/`PS4` but not `BASH_ENV`.

### Inferences

- Minimum strip list for a denylist approach (not recommended as the sole control): `BASH_ENV`, `ENV`, `SHELLOPTS`, `BASHOPTS`, `PS4`, `PS1`–`PS3`, `PROMPT_COMMAND`, every name matching `BASH_FUNC_*` (and the older Shellshock-era form where the value starts with `() {`), `IFS`, `CDPATH`, `GLOBIGNORE`, `ZDOTDIR`, `FPATH`, `HISTFILE`, `TMOUT`, `POSIXLY_CORRECT`, `LD_*`, `DYLD_*`, `PATH`, `HOME`, `SHELL`, `TMPDIR`. Interpreter equivalents for the still-allowed interpreters: `PYTHONPATH`, `PYTHONSTARTUP`, `PYTHONHOME`, `PERL5OPT`, `PERL5LIB`, `PERLLIB`, `NODE_OPTIONS` (`--require`), `NODE_PATH`, `RUBYOPT`, `RUBYLIB`. Windows: `PSModulePath`, `COMSPEC`, `PATHEXT`. The interpreter variables are general knowledge and were not individually sourced here.
- Better: `Command::env_clear()`, then add an explicit allowlist (e.g. `LANG`/`LC_*`, `TERM`, a bnto-controlled `PATH`, `HOME` if needed, plus recipe-declared `env` keys validated against a name regex and the denylist above). This is OWASP/CWE "accept known good" applied to the environment. The current approach strips only from the recipe's `env` param. If the _parent_ environment is inherited, the user's own `BASH_ENV`/`BASH_FUNC_*` still flow in. That is not an attacker vector per se, but it is a reproducibility hazard. It becomes an attack vector if any recipe or upstream node can set env (bnto code check needed).
- Invoke shells by absolute path (`/bin/sh`, `/bin/bash`, resolved once), not via PATH lookup. Add hardening flags: `bash -p` (works with uid == euid as a pure "ignore env imports" switch, per the manual) and `zsh -f`. ksh and dash have no BASH_FUNC import, but ksh needs `FPATH` cleared.
- Prefer `sh` as the default (fewer startup hooks), and make `bash`/`zsh` opt-in.

### Gaps

- On Linux bash 5.x, I did not test whether `BASH_FUNC_*` import is suppressed in posix mode / as `sh`. On macOS bash 3.2 it is _not_ suppressed. Debian/Ubuntu `/bin/sh` is dash, which does not import functions.
- Whether zsh uses `$HOME` from the passwd database when `HOME` is unset (affecting `.zshenv` lookup after bnto strips HOME) was not verified.
- The GNU manual pages were cited from canonical URLs. Their text was verified against the local bash 3.2 man page rather than re-fetched, so bash-5-specific wording (BASHOPTS/CDPATH/GLOBIGNORE in `-p`) should be double-checked against the online manual.

## 7. Notable CVEs: template/expression values injected into shell commands (excluding GitHub Actions)

### Takeaway

The recurring root cause is the same in each case: a templating or interpolation step runs _before_ a shell parses the resulting string. Budibase (low-code automation "bash step"), Apache Airflow (example DAGs with params), Coder (Terraform dotfiles module), Cloudflare Wrangler (CLI flag interpolated via JS template literal), and Cacti (host-variable substitution into a shell call with a no-op sanitizer) all fit this pattern. Fixes move data out of the code string (argv or env) or remove the shell.

### Cited Findings

- **CVE-2026-25044, Budibase < 3.33.4**: "the bash automation step executes user-provided commands using execSync without proper sanitization or validation. User input is processed through processStringSync which allows template interpolation, potentially allowing arbitrary command execution." CWE-78. This is the closest analogue to bnto (workflow automation node, template engine, shell). — [CVE.org record](https://cveawg.mitre.org/api/cve/CVE-2026-25044)
- **CVE-2022-24288, Apache Airflow < 2.2.4**: "some example DAGs did not properly sanitize user-provided params, making them susceptible to OS Command Injection from the web UI." CWE-78. Airflow's current docs prescribe `env` for templated values (see §2). — [CVE.org record](https://cveawg.mitre.org/api/cve/CVE-2022-24288), [Airflow docs](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)
- **CVE-2026-44454, Coder < 2.29.7 / 2.30.0–2.30.1**: "the `dotfiles` registry module passed unsanitized user input to shell commands". A crafted `dotfiles_uri` plus `mode=auto` deep links made it "a one-click attack" that "silently provisioned a workspace … with no explicit user confirmation". This shows a consent bypass amplifying injection. — [CVE.org record](https://cveawg.mitre.org/api/cve/CVE-2026-44454)
- **CVE-2026-0933, Cloudflare Wrangler** (`wrangler pages deploy`, fixed in 3.114.17 / 4.59.1): "Shell metacharacters are interpreted by the shell, enabling command execution". It requires an attacker-controlled `--commit-hash`. The search-result summary attributes it to JS template-literal interpolation, but the CVE text itself only says shell metacharacters are interpreted. — [CVE.org record](https://cveawg.mitre.org/api/cve/CVE-2026-0933)
- **CVE-2026-40079, Cacti ≤ 1.2.30**: per an aggregator, host variable substitutions feeding graph-template `text_format` reach a shell call, and `escape_command()` returns its input unchanged. Fixed in 1.2.31. _Aggregator only, not verified against the primary record._ — [SentinelOne vuln DB](https://www.sentinelone.com/vulnerability-database/cve-2026-40079/)
- **CVE-2024-24576 / CVE-2024-43402 (Rust std, BatBadBut)**: runtime-level argument escaping failure for `.bat`/`.cmd` (see §3). — [Rust blog](https://blog.rust-lang.org/2024/04/09/cve-2024-24576/), [GHSA-2xg3-7mm6-98jj](https://github.com/rust-lang/rust/security/advisories/GHSA-2xg3-7mm6-98jj)
- Historical argument-injection CVEs (CVE-2016-10033 PHPMailer, CVE-2022-36069) — see §1. — [CWE-88](https://cwe.mitre.org/data/definitions/88.html)

### Inferences

- Budibase and Airflow are the two closest precedents to bnto's model: a recipe/DAG author writes a shell step, and runtime data is templated in. Both were bitten by template-before-parse, and Airflow's documented fix is exactly the positional/env-channel design bnto proposes.
- Coder's case is relevant to bnto's consent model. Any path that installs or runs a recipe without showing the consent prompt (deep links, `--yes` flags, auto-import) becomes the amplifier.

### Gaps

- Primary records for Cacti CVE-2026-40079 were not fetched. Other well-known cases (Ansible `shell` module with `{{ var }}` vs `| quote`, Jenkins `sh` step Groovy string interpolation warnings, n8n Execute Command node) were not researched in this session.

## 8. Assessment of bnto's proposed `script` design: strengths and gaps

### Takeaway

The core mechanism is the industry- and standards-endorsed pattern: script text never templated, `{{` rejected, values only as positional parameters (POSIX) or env vars (PowerShell). It is the same pattern ShellCheck and Airflow prescribe, and it removes engine-caused injection. The remaining risks are (1) script-author hygiene the engine can't enforce, (2) environment/startup-file code execution before the script runs, (3) argument injection, (4) Windows batch and PowerShell specifics, and (5) inconsistency with the existing args path, which still allows templating into `python -c`/`osascript -e` code.

### Cited Findings

- The positional-parameter form is POSIX-defined and recommended by ShellCheck SC2156. — [POSIX sh](https://pubs.opengroup.org/onlinepubs/9799919799/utilities/sh.html), [ShellCheck SC2156](https://www.shellcheck.net/wiki/SC2156)
- Env-var data channel for templated values is Airflow's documented fix. — [Airflow docs](https://airflow.apache.org/docs/apache-airflow-providers-standard/stable/operators/bash.html)
- `-Command` concatenates trailing args into code. `-CommandWithArgs` (pwsh ≥ 7.4/7.5) provides `$args`. 5.1 lacks it. — [about_Pwsh](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pwsh), [about_PowerShell_exe](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1)
- bash `-c` sources `BASH_ENV`, and bash (also as `sh`) imports `BASH_FUNC_*` and honors `SHELLOPTS`+`PS4`. zsh `-c` sources `$ZDOTDIR/.zshenv`. `bash -p` and `zsh -f` suppress these. — [Bash Startup Files](https://www.gnu.org/software/bash/manual/html_node/Bash-Startup-Files.html), [The Set Builtin](https://www.gnu.org/software/bash/manual/html_node/The-Set-Builtin.html), [zsh Files](https://zsh.sourceforge.io/Doc/Release/Files.html); local tests in §6
- Argument injection survives structured argv. — [OWASP cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/OS_Command_Injection_Defense_Cheat_Sheet.html), [CWE-88](https://cwe.mitre.org/data/definitions/88.html)

### Inferences

**Strengths**

1. Data never enters the parser. With no templating in `script`, the Budibase/Airflow-class bug (template-before-parse) is eliminated at the engine level.
2. Rejecting `{{` in script is a clear, testable invariant. It also prevents authors from "accidentally" relying on templating and keeps the consent hash meaningful, since the script text is static.
3. A fixed shell allowlist (`sh|bash|zsh|dash|ksh`) is an allowlist, which is what OWASP asks for, unlike the current denylist.
4. Always supplying `<name>` as `$0` avoids the POSIX "first arg becomes $0" off-by-one, which would otherwise shift data.
5. The Windows env-var channel is the only channel that works on both 5.1 and 7.x without touching the command line.
6. Consent pinned to a SHA-256 of the definition puts author trust in the right place (Home Assistant and Airflow likewise treat the command author as trusted).

**Gaps and recommended hardening**

1. **Environment hygiene (highest priority).** Use `env_clear()` plus an allowlist rather than stripping six names. At minimum, add `BASH_ENV`, `ENV`, `SHELLOPTS`, `BASHOPTS`, `PS4`, `BASH_FUNC_*`, `IFS`, `CDPATH`, `GLOBIGNORE`, `ZDOTDIR`, `FPATH` to the strip list. Invoke `bash -p`, `zsh -f`, and shells by absolute path. Without this, a recipe `env` entry (or an inherited parent env) is a code-execution channel that bypasses the "script is the only code" invariant, and the consent hash would need to cover env _values_ too.
2. **Script-author hygiene is unenforced.** Unquoted `$1`, `eval "$1"`, nested `sh -c "…$1…"`, and arithmetic contexts re-introduce injection (local tests in §2). Mitigations: docs with a canonical safe example (`"$@"`, `"$1"`, `--`), an optional ShellCheck lint in validation or `bnto doctor`, and a recipe-validation warning for `eval` in scripts.
3. **Argument injection.** A value like `-rf` or `--upload-file` passed as `"$1"` is still an option. Document `--` usage and offer a field-level "must not start with `-`" validator or path normalization (`./`-prefix) for file inputs.
4. **Inconsistency with the existing args path.** Templating into the code argument of `python3 -c`, `node -e`, `perl -e`, `osascript -e` recreates the exact bug the `script` design forbids. Apply the same rule there (reject `{{` in the argument after `-c`/`-e`/`--eval` for known interpreters, and steer data to trailing argv). Otherwise, reconsider the shell denylist, which provides little security once `python3 -c` is allowed and becomes redundant once `script` exists.
5. **Windows.** Require Rust ≥ 1.81.0 (CVE-2024-43402). Surface `InvalidInput` from batch-shim spawns clearly. Consider refusing templated args to resolved `.bat`/`.cmd` shims, or requiring an explicit extension (BatBadBut guidance). For PowerShell, use `-NoProfile -NonInteractive -EncodedCommand <script>` plus `BNTO_*` env vars. Strip `PSModulePath`, `PATHEXT`, `COMSPEC`. Document that `Invoke-Expression`/`ExpandString`/`[scriptblock]::Create` on `$env:` values is unsafe. Optionally, use `-CommandWithArgs` on pwsh ≥ 7.5 for parity with `$1`, while keeping env vars as the portable channel.
6. **Shell choice.** Default to `sh`. `zsh` without `-f` silently runs user `.zshenv`, which hurts determinism, and `bash` without `-p` honors `BASH_ENV`. Note that `/bin/sh` differs by platform (bash-as-sh on macOS imports `BASH_FUNC_*`; dash on Debian/Ubuntu does not), so env hygiene matters even for `sh`.
7. **Consent scope.** The hash must cover the script text, shell choice, `cwd`, env keys and any literal env values, and args templates. Every path that can run a non-bundled recipe (deep links, `--yes`, TUI import) must go through consent. Coder CVE-2026-44454 shows consent-bypass amplification.
8. **Name/`$0`.** Use a fixed, non-data `$0` (e.g. `bnto-script` or the node id, never a templated value). Per SC2156, don't put data in `$0`.

### Gaps

- No authoritative source evaluates a design identical to bnto's. The assessment synthesizes POSIX/ShellCheck/OWASP/CWE guidance, the Airflow and Home Assistant precedents, and local shell tests.
- Not verified in bnto code: whether the parent environment is inherited by default, whether shells are resolved via PATH or by absolute path, how `.cmd` shims are resolved before spawn, and the Rust MSRV used for Windows release builds.
