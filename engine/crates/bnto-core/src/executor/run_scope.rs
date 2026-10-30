// A batch node's context: the consumer's, plus what the run knows that one
// node can't see. `held_elsewhere` answers from the run's held files and
// `record_write`/`recorded_writes` keep the run's write log, so a refusal in
// a later loop iteration can name what earlier ones copied (ISS-59).
// `check_time_limit` answers from the run's limit, so a batch node can stop
// between files (ISS-60). `loop_item` is the enclosing loop's item, and
// `run_placeholder` fills the run's own namespaces for the node, which a node
// that fills its params itself reads (ISS-58, ISS-35). Every other `ProcessContext` method forwards to the
// consumer's context unchanged.

use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Mutex;

use crate::command_options::CommandOptions;
use crate::context::ProcessContext;
use crate::errors::BntoError;

use super::PipelineContext;
use super::field_scope::FieldValues;
use super::held::HeldFiles;
use super::node_outputs::NodeOutputSource;
use super::template::{TemplateContext, run_placeholder};
use super::time_limit::{TimeLimit, check_limit};

/// The files this run has written, in order, shared by every node of the run.
pub(super) type WriteLog = Rc<Mutex<Vec<PathBuf>>>;

/// The consumer's context, with the run's held files, write log and time limit.
pub(super) struct RunScope<'a> {
    inner: &'a dyn ProcessContext,
    held: Option<&'a HeldFiles>,
    writes: &'a Mutex<Vec<PathBuf>>,
    time_limit: Option<TimeLimit>,
    now_ms: &'a (dyn Fn() -> u64 + Send + Sync),
    loop_item: Option<&'a serde_json::Map<String, serde_json::Value>>,
    /// What `run_placeholder` fills from: the node's fields in scope, the
    /// records of the nodes run so far, and the run's start.
    run: RunValues<'a>,
}

pub(super) struct RunValues<'a> {
    fields: FieldValues,
    node_outputs: &'a (dyn NodeOutputSource + Sync),
    started_ms: u64,
}

impl<'a> RunScope<'a> {
    /// The context the batch node `node` running in `ctx` gets.
    pub(super) fn of<F: Fn() -> u64 + Copy>(
        ctx: &'a PipelineContext<'_, F>,
        node: &crate::pipeline::PipelineNode,
    ) -> Self {
        let run = RunValues {
            fields: ctx.fields_for(node),
            node_outputs: &ctx.node_outputs,
            started_ms: ctx.started_ms,
        };
        Self {
            inner: ctx.process_ctx,
            held: ctx.held.as_deref(),
            writes: &ctx.writes,
            time_limit: ctx.time_limit,
            now_ms: &*ctx.live_clock,
            loop_item: ctx.loop_item.as_ref(),
            run,
        }
    }
}

impl ProcessContext for RunScope<'_> {
    // --- The run ---

    fn held_elsewhere(&self, path: &Path) -> Option<String> {
        self.held?.reader_of(path)
    }

    fn record_write(&self, path: &Path) {
        let mut writes = self.writes.lock().unwrap_or_else(|e| e.into_inner());
        writes.push(path.to_path_buf());
    }

    fn recorded_writes(&self) -> Vec<PathBuf> {
        self.writes
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    fn check_time_limit(&self) -> Result<(), BntoError> {
        check_limit(self.time_limit, self.now_ms)
    }

    fn loop_item(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.loop_item.cloned()
    }

    fn run_placeholder(&self, inner: &str, whole: bool) -> Option<serde_json::Value> {
        let tpl = TemplateContext {
            field_values: &self.run.fields,
            process_ctx: self.inner,
            node_outputs: self.run.node_outputs,
            loop_item: &None,
            run_started_ms: Some(self.run.started_ms),
        };
        run_placeholder(inner, whole, &tpl)
    }

    // --- Forwarded to the consumer's context ---

    fn run_command(&self, cmd: &str, args: &[&str]) -> Result<Vec<u8>, BntoError> {
        self.inner.run_command(cmd, args)
    }

    fn run_command_streaming(
        &self,
        cmd: &str,
        args: &[&str],
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        self.inner.run_command_streaming(cmd, args, on_output)
    }

    fn run_command_with(
        &self,
        cmd: &str,
        args: &[&str],
        options: &CommandOptions,
        on_output: &dyn Fn(&str),
    ) -> Result<Vec<u8>, BntoError> {
        self.inner.run_command_with(cmd, args, options, on_output)
    }

    fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
        self.inner.temp_file(suffix)
    }

    fn env_var(&self, key: &str) -> Option<String> {
        self.inner.env_var(key)
    }

    fn work_dir(&self) -> Result<&Path, BntoError> {
        self.inner.work_dir()
    }

    fn find_binary(&self, name: &str) -> Option<PathBuf> {
        self.inner.find_binary(name)
    }

    fn home_dir(&self) -> Option<&Path> {
        self.inner.home_dir()
    }

    fn output_dir(&self) -> Option<PathBuf> {
        self.inner.output_dir()
    }

    fn dry_run(&self) -> bool {
        self.inner.dry_run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A consumer's context whose every method gives a value of its own,
    /// so a test can tell it was asked.
    struct Marked {
        dir: PathBuf,
    }

    impl ProcessContext for Marked {
        fn run_command(&self, cmd: &str, _: &[&str]) -> Result<Vec<u8>, BntoError> {
            Ok(format!("run {cmd}").into_bytes())
        }
        fn run_command_streaming(
            &self,
            cmd: &str,
            _: &[&str],
            _: &dyn Fn(&str),
        ) -> Result<Vec<u8>, BntoError> {
            Ok(format!("stream {cmd}").into_bytes())
        }
        fn run_command_with(
            &self,
            cmd: &str,
            _: &[&str],
            options: &CommandOptions,
            _: &dyn Fn(&str),
        ) -> Result<Vec<u8>, BntoError> {
            Ok(format!("with {cmd} {}", options.env.len()).into_bytes())
        }
        fn temp_file(&self, suffix: &str) -> Result<PathBuf, BntoError> {
            Ok(self.dir.join(format!("temp{suffix}")))
        }
        fn env_var(&self, key: &str) -> Option<String> {
            Some(format!("env {key}"))
        }
        fn work_dir(&self) -> Result<&Path, BntoError> {
            Ok(&self.dir)
        }
        fn find_binary(&self, name: &str) -> Option<PathBuf> {
            Some(self.dir.join(name))
        }
        fn home_dir(&self) -> Option<&Path> {
            Some(&self.dir)
        }
        fn output_dir(&self) -> Option<PathBuf> {
            Some(self.dir.join("out"))
        }
        fn dry_run(&self) -> bool {
            true
        }
        fn held_elsewhere(&self, _: &Path) -> Option<String> {
            Some("the consumer's".to_string())
        }
        fn recorded_writes(&self) -> Vec<PathBuf> {
            vec![self.dir.join("the consumer's")]
        }
        fn check_time_limit(&self) -> Result<(), BntoError> {
            Err(BntoError::TimedOut { secs: 1 })
        }
        fn loop_item(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
            Some(serde_json::Map::from_iter([(
                "consumer".to_string(),
                true.into(),
            )]))
        }
    }

    fn text(result: Result<Vec<u8>, BntoError>) -> String {
        String::from_utf8(result.unwrap()).unwrap()
    }

    #[test]
    fn test_every_consumer_method_is_forwarded() {
        // A method added to `ProcessContext` must be forwarded here, or a
        // batch node silently gets its default instead of the consumer's.
        let dir = PathBuf::from("work");
        let marked = Marked { dir: dir.clone() };
        let writes = Mutex::default();
        let scope = RunScope::unlimited(&marked, &writes);
        let options = CommandOptions {
            env: vec![("K".into(), "V".into())],
            ..CommandOptions::default()
        };
        assert_eq!(text(scope.run_command("ls", &[])), "run ls");
        assert_eq!(
            text(scope.run_command_streaming("ls", &[], &|_| {})),
            "stream ls"
        );
        let with = scope.run_command_with("ls", &[], &options, &|_| {});
        assert_eq!(text(with), "with ls 1");
        assert_eq!(scope.temp_file(".txt").unwrap(), dir.join("temp.txt"));
        assert_eq!(scope.env_var("HOME").as_deref(), Some("env HOME"));
        assert_eq!(scope.work_dir().unwrap(), dir);
        assert_eq!(scope.find_binary("git"), Some(dir.join("git")));
        assert_eq!(scope.home_dir(), Some(dir.as_path()));
        assert_eq!(scope.output_dir(), Some(dir.join("out")));
        assert!(scope.dry_run());
    }

    struct NoRecords;

    impl NodeOutputSource for NoRecords {
        fn node_output(&self, _: &str) -> Option<&serde_json::Value> {
            None
        }
    }

    impl<'a> RunScope<'a> {
        /// A scope over `inner` with nothing held and no time limit.
        fn unlimited(inner: &'a dyn ProcessContext, writes: &'a Mutex<Vec<PathBuf>>) -> Self {
            Self {
                inner,
                held: None,
                writes,
                time_limit: None,
                now_ms: &|| 0,
                loop_item: None,
                run: RunValues {
                    fields: FieldValues::new(),
                    node_outputs: &NoRecords,
                    started_ms: 0,
                },
            }
        }
    }

    #[test]
    fn test_the_run_answers_for_itself_not_the_consumer() {
        let marked = Marked {
            dir: PathBuf::from("work"),
        };
        let writes = Mutex::default();
        let scope = RunScope::unlimited(&marked, &writes);
        assert!(scope.held_elsewhere(Path::new("a.txt")).is_none());
        scope.record_write(Path::new("out/a.txt"));
        assert_eq!(scope.recorded_writes(), [PathBuf::from("out/a.txt")]);
        assert!(
            scope.check_time_limit().is_ok(),
            "no limit, whatever the consumer says"
        );
        assert!(
            scope.loop_item().is_none(),
            "no loop, whatever the consumer says"
        );
        let item = serde_json::Map::from_iter([("row".to_string(), "r1".into())]);
        let in_loop = RunScope {
            loop_item: Some(&item),
            ..RunScope::unlimited(&marked, &writes)
        };
        assert_eq!(in_loop.loop_item(), Some(item.clone()));
    }

    #[test]
    fn test_the_run_fills_its_namespaces_for_the_node() {
        let marked = Marked {
            dir: PathBuf::from("work"),
        };
        let writes = Mutex::default();
        let mut scope = RunScope::unlimited(&marked, &writes);
        scope.run.fields.insert("n".to_string(), 2.into());
        assert_eq!(scope.run_placeholder("fields.n", true), Some(2.into()));
        assert_eq!(scope.run_placeholder("fields.n", false), Some("2".into()));
        assert_eq!(scope.run_placeholder("fields.gone", true), None);
        let env = scope.run_placeholder("env.HOME", false);
        assert_eq!(env, Some("env HOME".into()), "from the consumer's env");
        assert_eq!(
            scope.run_placeholder("item.k", false),
            None,
            "items are the node's"
        );
    }

    fn at_3s() -> u64 {
        3_000
    }

    fn at_1s() -> u64 {
        1_000
    }

    #[test]
    fn test_the_run_says_when_its_time_is_up() {
        // RED (ISS-60): a batch node asks between files; past the run's
        // limit it gets the executor's own timeout.
        let marked = Marked {
            dir: PathBuf::from("work"),
        };
        let writes = Mutex::default();
        let limit = TimeLimit::starting_at(0, Some(2));
        let scope = |now_ms: &'static (dyn Fn() -> u64 + Send + Sync)| RunScope {
            time_limit: limit,
            now_ms,
            ..RunScope::unlimited(&marked, &writes)
        };
        assert!(scope(&at_1s).check_time_limit().is_ok());
        let err = scope(&at_3s).check_time_limit().unwrap_err();
        assert_eq!(err.to_string(), "Run timed out after 2s");
        assert!(err.is_timeout());
    }
}
