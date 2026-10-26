// Results-screen actions — open an output file or folder with the OS.

use super::super::app::AppModel;

/// Open the selected output file in the default app: the row's own file,
/// wherever the run left it.
pub(crate) fn handle_open_file(model: AppModel) -> AppModel {
    if let Some(path) = model.results.as_ref().and_then(|r| r.selected_path()) {
        let _ = open_path(path.to_string_lossy().as_ref());
    }
    model
}

/// Open the output folder (for files written in place, the selected file's
/// folder) in the system file manager.
pub(crate) fn handle_open_folder(model: AppModel) -> AppModel {
    if let Some(dir) = model.results.as_ref().and_then(|r| r.folder_to_open()) {
        let _ = open_path(dir.to_string_lossy().as_ref());
    }
    model
}

/// Open a file or directory using the platform default handler.
fn open_path(path: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(path).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer").arg(path).spawn()?;
    }
    Ok(())
}
