// Results-screen actions — open an output file or folder with the OS.

use super::super::app::AppModel;

/// Open the selected output file in the default app.
pub(crate) fn handle_open_file(model: AppModel) -> AppModel {
    if let Some(results) = &model.results
        && let Some(dir) = &results.output_dir
        && let Some(file) = results.outputs.get(results.cursor)
    {
        let path = std::path::Path::new(dir).join(&file.name);
        let _ = open_path(path.to_string_lossy().as_ref());
    }
    model
}

/// Open the output folder in the system file manager.
pub(crate) fn handle_open_folder(model: AppModel) -> AppModel {
    if let Some(results) = &model.results
        && let Some(dir) = &results.output_dir
    {
        let _ = open_path(dir);
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
