// The folder a recipe's output node names in its `directory` param, with
// every placeholder filled. Shared by `bnto run` and the TUI so both write a
// recipe to the same place, and both refuse one nothing fills.

use bnto_core::{PipelineDefinition, PipelineFile, ProcessContext};

use crate::display_text::display_text;

/// The recipe's own output folder, a relative one under the run's working
/// folder, or `None` when its output node names none (or the JSON doesn't
/// parse, which the run refuses on its own). An `Err` names each placeholder
/// nothing fills, so the caller stops before it makes the folder or runs a
/// step.
pub fn recipe_output_dir(
    definition_json: &str,
    ctx: &dyn ProcessContext,
    files: &[PipelineFile],
) -> Result<Option<String>, String> {
    let Ok(def) = serde_json::from_str::<PipelineDefinition>(definition_json) else {
        return Ok(None);
    };
    let Some(dir) = bnto_core::fill_output_directory(&def, ctx, files)? else {
        return Ok(None);
    };
    in_work_dir(&dir, ctx).map(Some)
}

/// Where a run writes when neither `-o` nor the recipe names a folder: the
/// run's working folder, in full, so it follows the working folder as the
/// recipe's own paths do (step 7g runs a project recipe at its root).
pub fn default_output_dir(ctx: &dyn ProcessContext) -> Result<String, String> {
    let work = ctx.work_dir().map_err(|e| e.to_string())?;
    let work = std::path::absolute(work).map_err(|e| format!("Output folder: {e}"))?;
    as_text(work)
}

/// `dir` under the run's working folder, as text: the write path takes the
/// folder as a string, so a working folder that isn't UTF-8 is refused
/// rather than written to under a mangled name. A refusal holds the
/// recipe's text, so it's escaped (`display_text`) here: the TUI splits an
/// error into rows at each line break. The CLI's own pass leaves escaped
/// text as it is.
fn in_work_dir(dir: &str, ctx: &dyn ProcessContext) -> Result<String, String> {
    let path = bnto_core::in_work_dir("directory", dir, ctx)
        .map_err(|e| display_text(&e.to_string()).into_owned())?;
    as_text(path)
}

/// `path` as text, or a refusal naming it (escaped) when it isn't UTF-8.
fn as_text(path: std::path::PathBuf) -> Result<String, String> {
    path.into_os_string().into_string().map_err(|path| {
        let lossy = path.to_string_lossy();
        let shown = display_text(&lossy);
        format!("Output folder {shown} isn't valid UTF-8; pass -o to choose one")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::NativeContext;
    use crate::storage::BntoPaths;

    fn recipe(directory: &str) -> String {
        serde_json::json!({
            "id": "r", "name": "R",
            "nodes": [
                { "id": "input", "type": "input", "parameters": { "mode": "file-upload" } },
                { "id": "output", "type": "output",
                  "parameters": { "mode": "write", "directory": directory } }
            ]
        })
        .to_string()
    }

    /// A run whose working folder is a temp dir, not the test's own folder.
    fn in_work(test: impl FnOnce(&NativeContext, &std::path::Path)) {
        let (work, home) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let paths = BntoPaths {
            home: home.path().to_path_buf(),
        };
        let ctx = NativeContext::new(work.path().to_path_buf(), &paths);
        test(&ctx, work.path());
    }

    #[test]
    fn a_relative_directory_is_under_the_work_dir() {
        in_work(|ctx, work| {
            let dir = recipe_output_dir(&recipe("out/sub"), ctx, &[]).unwrap();
            let expected = work.join("out").join("sub");
            assert_eq!(dir.map(std::path::PathBuf::from), Some(expected));
        });
    }

    #[test]
    fn an_absolute_directory_is_kept() {
        in_work(|ctx, _| {
            let elsewhere = tempfile::tempdir().unwrap();
            let abs = elsewhere.path().join("out");
            let dir = recipe_output_dir(&recipe(abs.to_str().unwrap()), ctx, &[]).unwrap();
            assert_eq!(dir.as_deref(), abs.to_str());
        });
    }

    #[test]
    fn a_root_or_drive_relative_directory_is_refused_naming_it() {
        in_work(|ctx, _| {
            for dir in ["\\out", "C:out"] {
                let err = recipe_output_dir(&recipe(dir), ctx, &[]).unwrap_err();
                assert!(err.contains("'directory'"), "{dir}: {err}");
            }
        });
    }

    #[test]
    fn the_default_folder_is_the_work_dir_in_full() {
        // With no -o and no recipe folder, `bnto run` writes to its working
        // folder, not the process's own.
        in_work(|ctx, work| {
            let dir = default_output_dir(ctx).unwrap();
            assert_eq!(std::path::PathBuf::from(dir), work);
        });
    }

    #[test]
    fn no_directory_is_none() {
        in_work(|ctx, _| {
            assert_eq!(recipe_output_dir(&recipe(""), ctx, &[]).unwrap(), None);
        });
    }
}
