// The bridge's pipeline reporter: forwards engine events to the TUI and, like
// `bnto run`'s reporter, copies each finished loop iteration's files out on
// the pipeline's own thread. So every copy is on disk before the run returns
// and the bridge plans the results listing and the final write, whose made-up
// names depend on what's already in the folder (see `output_names`).

use std::cell::RefCell;
use std::sync::mpsc;

use bnto_core::events::{PipelineEvent, PipelineReporter};

use super::BridgeEvent;
use crate::output_names::OutputNames;
use crate::progressive_output::write_progressive_files;

/// The folder progressive output goes to, as `bnto run` decides it: none on a
/// preview or an overwrite (files are renamed in place, after the run).
pub fn progressive_dir(preview_mode: bool, mode: &str, output_dir: &str) -> Option<String> {
    (!preview_mode && mode != "overwrite").then(|| output_dir.to_string())
}

/// A reporter that sends every engine event to the TUI, first copying a
/// finished iteration's files into `progressive` when there is one.
pub fn channel_reporter(
    tx: &mpsc::Sender<BridgeEvent>,
    progressive: Option<String>,
) -> PipelineReporter {
    let progress_tx = tx.clone();
    // A new run: what an earlier one in this session wrote to the folder
    // isn't this run's, so a refusal won't list it (`ordered_write::check`).
    if let Some(dir) = &progressive {
        crate::ordered_write::forget_progressive(std::path::Path::new(dir));
    }
    // RefCell: the reporter is `Fn` and only called on the bridge thread.
    let names = RefCell::new(OutputNames::default());
    PipelineReporter::new(move |event| {
        if let (Some(dir), PipelineEvent::IterationCompleted { output_files, .. }) =
            (&progressive, &event)
        {
            write_progressive_files(dir, output_files, &mut names.borrow_mut());
        }
        let _ = progress_tx.send(BridgeEvent::Progress(event));
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use bnto_core::FileData;
    use bnto_core::events::ProgressOutputFile;

    fn iteration_done(src: &std::path::Path) -> PipelineEvent {
        PipelineEvent::IterationCompleted {
            node_id: "each".into(),
            iteration: 0,
            total_iterations: 1,
            duration_ms: 1,
            files_produced: 1,
            output_files: vec![ProgressOutputFile {
                name: "renamed.mp4".into(),
                data: FileData::Path(src.to_path_buf()),
                mime_type: "video/mp4".into(),
            }],
        }
    }

    #[test]
    fn an_iterations_files_are_copied_before_the_event_is_sent() {
        // The copy used to run on the UI thread whenever it drained the
        // event, racing the bridge's final write. Now it's on disk once
        // `emit` returns, the source stays (copy, never move), and the event
        // still goes out.
        let temp = tempfile::tempdir().unwrap();
        let src = temp.path().join("original.mp4");
        std::fs::write(&src, b"video").unwrap();
        let out = temp.path().join("out");
        std::fs::create_dir(&out).unwrap();
        let (tx, rx) = mpsc::channel();
        let reporter = channel_reporter(&tx, Some(out.to_string_lossy().into_owned()));

        reporter.emit(iteration_done(&src));

        assert_eq!(std::fs::read(out.join("renamed.mp4")).unwrap(), b"video");
        assert!(src.exists(), "source must survive a progressive write");
        let sent = rx.try_recv().unwrap();
        assert!(matches!(
            sent,
            BridgeEvent::Progress(PipelineEvent::IterationCompleted { .. })
        ));
    }

    #[test]
    fn a_new_run_forgets_what_earlier_runs_wrote_to_its_folder() {
        // The TUI runs many recipes in one process. A refusal lists the files
        // this run's iterations wrote; an earlier run's must not show up.
        let temp = tempfile::tempdir().unwrap();
        let earlier = temp.path().join("earlier.txt");
        crate::ordered_write::note_progressive(&earlier);
        let paths = [earlier];
        assert_eq!(crate::ordered_write::already_written(&paths).len(), 1);

        let (tx, _rx) = mpsc::channel();
        let dir = temp.path().to_string_lossy().into_owned();
        let _reporter = channel_reporter(&tx, Some(dir));

        assert!(crate::ordered_write::already_written(&paths).is_empty());
    }

    #[test]
    fn previews_and_overwrites_copy_nothing_as_they_run() {
        assert_eq!(progressive_dir(true, "write", "/out"), None);
        assert_eq!(progressive_dir(false, "overwrite", "/src"), None);
        assert_eq!(
            progressive_dir(false, "write", "/out").as_deref(),
            Some("/out")
        );
    }
}
