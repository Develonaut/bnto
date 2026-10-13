// TUI messages — every event that can drive an `AppModel` transition.

use super::super::screens::browser::BrowserMessage;
use super::super::screens::detail_scroll::DetailScrollMsg;
use super::super::screens::editor::EditorMessage;
use super::super::screens::execution::ExecutionMessage;
use super::super::screens::home::HomeMessage;
use super::super::screens::library::LibraryMessage;
use super::super::screens::picker::PickerMessage;
use super::super::screens::preview::PreviewMessage;
use super::super::screens::results::ResultsMessage;
use super::super::screens::settings::SettingsMessage;
use super::super::screens::wizard::WizardMessage;
use super::super::theme::ThemeVariant;

/// Messages that drive screen transitions.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AppMessage {
    /// User selected a recipe in the browser.
    RecipeSelected { slug: String },
    /// User confirmed config and moves to file picker.
    ConfigConfirmed { slug: String },
    /// User selected files and starts execution.
    FilesSelected { slug: String },
    /// Execution finished, show results.
    ExecutionComplete { slug: String },
    /// Navigate back one screen.
    Back,
    /// User wants to run another recipe (from results).
    RunAnother,
    /// Open the settings screen.
    OpenSettings,
    /// User selected a new theme in settings.
    ThemeChanged(ThemeVariant),
    /// User toggled telemetry on/off in settings.
    TelemetryToggled(bool),
    /// Forward a message to the home screen.
    Home(HomeMessage),
    /// User confirmed the home screen selection.
    HomeConfirm,
    /// Navigate to the Library screen.
    OpenLibrary,
    /// Forward a message to the library screen.
    Library(LibraryMessage),
    /// User confirmed a library selection.
    LibraryConfirm,
    /// Add the currently focused browser recipe to the user's library.
    AddToLibrary,
    /// User confirmed overwriting an existing library recipe.
    AddToLibraryConfirm { slug: String },
    /// Forward a message to the browser screen.
    Browser(BrowserMessage),
    /// Forward a form message to the detail screen's FormModel.
    DetailForm(tonkotsu::FormMessage),
    /// Scroll the detail screen viewport (whole-screen scroll).
    DetailScroll(DetailScrollMsg),
    /// Forward a message to the detail screen's embedded picker.
    DetailPicker(PickerMessage),
    /// Move detail focus to Input section.
    DetailFocusInput,
    /// Move detail focus to Params section.
    DetailFocusParams,
    /// Forward a message to the picker screen.
    Picker(PickerMessage),
    /// Forward a message to the execution screen.
    Execution(ExecutionMessage),
    /// Forward a message to the results screen.
    Results(ResultsMessage),
    /// Forward a message to the preview screen.
    Preview(PreviewMessage),
    /// User requested preview mode from the detail screen (p key on Run).
    PreviewRequested { slug: String },
    /// User confirmed the preview — re-run with writes.
    PreviewConfirm { slug: String },
    /// Forward a message to the settings screen.
    Settings(SettingsMessage),
    /// Forward a message to the editor screen.
    Editor(EditorMessage),
    /// Forward a form message to the editor's inline form.
    EditorForm(tonkotsu::FormMessage),
    /// Forward a message to the wizard screen.
    Wizard(WizardMessage),
    /// Forward a form message to the wizard's inline form.
    WizardForm(tonkotsu::FormMessage),
    /// Open the wizard for guided recipe creation.
    OpenWizard,
    /// Open the editor for a predefined recipe (clone into editor).
    OpenEditorFromBrowser,
    /// Open the editor for a library recipe (edit in place).
    OpenEditorFromLibrary,
    /// Open the file picker from settings to browse for a directory.
    OpenSettingsPicker { field_key: String },
    /// Confirm the current picker directory as the settings field value.
    SettingsPathConfirmed,
    /// Open the currently selected output file in the default app.
    OpenFile,
    /// Open the output folder in the system file manager.
    OpenFolder,
    /// Quit the application.
    Quit,
}
