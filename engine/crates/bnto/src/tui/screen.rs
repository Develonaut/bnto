// Screen presentation — display strings and help hints per screen.
//
// Keeps rendering concerns separate from the state machine in app.rs.

use super::app::Screen;

impl Screen {
    /// Display title for the screen's border block.
    ///
    /// Returns plain names — the `content_panel()` helper handles
    /// spacing and padding at render time.
    pub fn title(&self) -> &'static str {
        match self {
            Self::Home => "bnto",
            Self::Library => "Library",
            Self::Browser => "Recipes",
            Self::Detail { .. } => "Detail",
            Self::Picker { .. } => "File Picker",
            Self::Execution { .. } => "Running",
            Self::Results { .. } => "Results",
            Self::Preview { .. } => "Preview",
            Self::Settings => "Settings",
            Self::Editor { .. } => "Editor",
            Self::Wizard { .. } => "Wizard",
        }
    }

    /// Short label for log entries (no dynamic data).
    pub fn log_label(&self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Library => "library",
            Self::Browser => "browser",
            Self::Detail { .. } => "detail",
            Self::Picker { .. } => "picker",
            Self::Execution { .. } => "execution",
            Self::Results { .. } => "results",
            Self::Preview { .. } => "preview",
            Self::Settings => "settings",
            Self::Editor { .. } => "editor",
            Self::Wizard { .. } => "wizard",
        }
    }

    /// Contextual key hints for the help bar.
    pub fn help_hints(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            Self::Home => vec![
                ("Tab", "pane"),
                ("↑↓", "navigate"),
                ("Enter", "select"),
                ("q", "quit"),
            ],
            Self::Library => vec![
                ("↑↓", "navigate"),
                ("/", "search"),
                ("Enter", "select"),
                ("r", "rename"),
                ("d", "delete"),
                ("Esc", "back"),
            ],
            Self::Browser => vec![
                ("↑↓", "navigate"),
                ("/", "search"),
                ("Enter", "select"),
                ("a", "add to library"),
                ("s", "settings"),
                ("q", "quit"),
            ],
            Self::Detail { .. } => vec![
                ("↑↓", "navigate"),
                ("Enter", "edit/select"),
                ("Tab", "next section"),
                ("Space", "select file"),
                ("Esc", "back"),
            ],
            Self::Picker { .. } => vec![
                ("↑↓", "navigate"),
                ("h/l", "parent/enter"),
                ("Space", "select"),
                ("a", "all"),
                ("/", "search"),
                ("p", "metadata"),
                (".", "hidden"),
                ("Esc", "back"),
            ],
            Self::Execution { .. } => vec![("Esc", "cancel"), ("Enter", "continue")],
            Self::Results { .. } => vec![
                ("o", "open file"),
                ("O", "open folder"),
                ("r", "run another"),
                ("q", "quit"),
            ],
            Self::Preview { .. } => vec![("↑↓", "navigate"), ("Enter", "confirm"), ("Esc", "back")],
            Self::Settings => vec![
                ("↑↓", "navigate"),
                ("◂▸", "theme"),
                ("Enter", "browse"),
                ("Esc", "back"),
            ],
            Self::Editor { .. } => vec![
                ("↑↓", "navigate"),
                ("Enter", "expand"),
                ("a", "add"),
                ("d", "delete"),
                ("J/K", "reorder"),
                ("u", "undo"),
                ("Esc", "back"),
            ],
            Self::Wizard { .. } => vec![
                ("↑↓", "navigate"),
                ("Enter", "select"),
                ("Tab", "confirm"),
                ("Esc", "back"),
            ],
        }
    }
}
