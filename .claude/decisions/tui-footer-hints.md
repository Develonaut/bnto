# Decision: the TUI footer composes its key hints from a mode, the screen and a base set

**Created:** October 27, 2026
**Status:** Decided by Ryan on October 27, 2026: "it's supposed to be configurable per screen and have like a base set so it's extensable". Built for ISS-101 in `engine/crates/bnto/src/tui/footer_hints.rs`.
**Related:** [tui-strategy.md](../strategy/tui-strategy.md) (footer styling), [tui-controls-bubbles.md](../strategy/tui-controls-bubbles.md) (Bubbles' auto-generated help, the longer-term idea), [gotchas.md](../rules/gotchas.md)

## The problem

Each screen hard-coded its whole hint list in `Screen::help_hints`. Keys that work everywhere were copied by hand, and inconsistently: `q quit` was listed on Home, Browser and Results but not on Library, Detail, Picker, Preview, Execution, Settings, Editor or Wizard, though `q` quits from all of them. Modes that change the keys had no place to say so: the editor kept showing `↑↓ navigate  Enter expand  a add` while its delete or save question took only `y`, `n` and Esc (ISS-101), the library did the same during its delete question and a rename, and consent and a run in progress showed `Enter continue`, which does nothing until the run ends. The settings picker's hints were a one-off `if` in `draw_bottom_bar`.

## The design

`footer_hints(model)` builds the list from three layers, each extended in one place:

1. **A mode's keys, or else the screen's own.** A mode is a state that changes which keys work (`mode_hints`): the editor's and the library's delete questions, the editor's save question, a library rename, consent, a run in progress, and the settings picker. Without one, `Screen::help_hints` gives the screen's own keys, with nothing every screen shares.
2. **Then the base keys** (`BASE`: `q quit`), which work on every screen.
3. **Except where a mode takes every key.** The key router's own test (`keys::captures_keys`, the `capturing_handler` that sends every key to a search box, a question or a field being edited) decides it, so the footer can't disagree with the keys: in a search box `q` is typed, and in a question it's no answer.

The footer drops hints from the end when it's narrow, so the screen's or mode's keys outlast the base ones. At 80 columns the status leaves about 25 columns, so a mode's hints are kept short (`y save  n discard  Esc stay`, `y allow  n/Esc cancel`) to fit whole.

## Extending it

- **A key every screen has:** add it to `BASE`.
- **A screen's key:** add it to that screen's arm of `Screen::help_hints`.
- **A new mode:** add a branch to `mode_hints`. If the mode takes every key, it's already in `capturing_handler`, and the base keys hide on their own.

## Not yet covered

Search boxes (Browser, Picker, Library) and field editing (Editor, Detail, Wizard), and the editor's node picker, capture every key too. The footer hides `q` there but still shows the screen's normal hints. Each needs its own short set (ISS-102).

## Alternatives considered

- **Keep hard-coded lists and fix each screen by hand.** Rejected: the copies drift, as `q quit` had.
- **Generate hints from the key handlers (Bubbles' `help.Model` from a keymap).** The better end state, since a hint could never name a key that does nothing, but it means turning every `keys/*.rs` `match` into a declared keymap. Too large for ISS-101. This design keeps the hints in one module, so a keymap could replace its layers later.
