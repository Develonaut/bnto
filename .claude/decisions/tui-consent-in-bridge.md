# Decision: The TUI Asks for Consent from the Run's Bridge

**Created:** October 26, 2026
**Status:** Decided by Ryan on October 26, 2026 (ISS-77, ISS-78).
**Applies to:** `engine/crates/bnto/src/tui/bridge/precheck.rs`, `tui/run_loop/bridge_events.rs`, `tui/app_helpers/consent.rs`.

---

## The problem

`bnto run` checks a run before it asks to allow a shell recipe's commands: it fills `{{vars.*}}` (SEC-29), refuses an empty field or a param a step won't take (ISS-74), prepares the inputs and fills the output folder. Only then does it ask, so nobody approves a run that is then refused.

The TUI asked first. `consent::gate` held the run on the UI thread when a run started, and the checks ran afterwards, on the bridge thread. An untrusted recipe that would be refused showed the prompt, and allowing it ended in an error (ISS-77). The TUI also never filled `{{vars.*}}` at all (ISS-78).

The checks can't simply move to the UI thread. Preparing inputs walks picked folders, and the output folder's placeholders are filled from the prepared files. Both have to stay off the UI thread.

## Options

1. **The bridge holds consent (chosen).** The run's bridge starts at once and does every check `bnto run` does, in its order. It then decides consent on the vars-filled definition. If the run needs consent, the bridge sends `BridgeEvent::ConsentNeeded` with a reply channel and waits. Allow sends on the reply and the same run goes on. Declining leaves the screen; the run loop lets the bridge go, which drops the reply, and the bridge stops without reporting anything.
2. **Cheap checks on the UI thread.** Fill vars and check fields before the old gate, but leave the input and output-folder refusals after Allow. This is smaller, but it leaves part of ISS-77 open and checks the run in two places.

## Why the bridge

- **One code path.** The checks and the run share one path, so the TUI can't drift from `bnto run`'s order. Inputs are prepared once.
- **No folder walk on the UI thread.**
- **Approval matches `bnto run`.** It is pinned to the filled values (SEC-29), and nothing, including the output folder, is made before consent.

**The cost:** the prompt now appears after a short check instead of at once, and SEC-19's tests changed shape. The UI side now checks what each start path hands the bridge (`consent_check`). The bridge's tests prove it asks only after its checks, waits, and stops when declined. The run loop's tests prove the request holds the run and Allow answers it.

## Rules

- **Keep the order.** A check `bnto run` makes before consent goes in `precheck::check_then_consent`, before `allowed`.
- **Nothing that changes the disk happens before consent.** The output folder is filled there but made only after (`resolve_output_dir`).
- **"Bundled" is decided on the UI thread,** against the exact shipped definition, because only the catalog knows it. No bundled recipe uses `{{vars.*}}`, so filling vars can't change that answer.
