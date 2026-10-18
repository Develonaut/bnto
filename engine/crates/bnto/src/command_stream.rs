// Command streaming — relay a running child's stdout and stderr lines to a
// callback as they arrive, collect them, and stop the child if it outlives
// its time limit. Lines are read on two threads and relayed on the caller's,
// so tools like yt-dlp show progress live while stdout is still captured.

use std::io::{BufRead, Read};
use std::process::{Child, ExitStatus};
use std::sync::mpsc::{Receiver, TryRecvError, channel};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// How long the relay loop sleeps when neither stream has a line ready.
const IDLE_POLL: Duration = Duration::from_millis(10);

/// What a command left when it ended, or was stopped for running too long.
pub(crate) struct Finished {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<String>,
    /// True when `kill` ran because the command outlived its timeout.
    pub timed_out: bool,
}

/// One stream's lines as they arrive, and its reader thread's collected output.
struct Stream<T> {
    lines: Receiver<String>,
    reader: JoinHandle<T>,
}

/// Relay `child`'s output to `on_output` until both streams close. Calls
/// `kill` once if the command is still running after `timeout`; killing it
/// closes its streams, which ends the relay.
pub(crate) fn stream(
    child: &mut Child,
    on_output: &dyn Fn(&str),
    timeout: Option<Duration>,
    kill: &dyn Fn(),
) -> std::io::Result<Finished> {
    let stdout = read_lines(child.stdout.take().expect("stdout piped"), keep_bytes);
    let stderr = read_lines(child.stderr.take().expect("stderr piped"), keep_lines);
    let deadline = timeout.map(|t| Instant::now() + t);
    let timed_out = relay(&stdout.lines, &stderr.lines, on_output, deadline, kill);
    let status = child.wait()?;
    Ok(Finished {
        status,
        stdout: join(stdout.reader, "stdout")?,
        stderr: join(stderr.reader, "stderr")?,
        timed_out,
    })
}

/// Read `pipe` line by line on a thread, sending each line on and folding
/// them into the thread's result with `keep`.
fn read_lines<R, T>(pipe: R, keep: fn(&mut T, &str)) -> Stream<T>
where
    R: Read + Send + 'static,
    T: Default + Send + 'static,
{
    let (tx, lines) = channel::<String>();
    let reader = std::thread::spawn(move || {
        let mut kept = T::default();
        for line in std::io::BufReader::new(pipe).lines().map_while(Result::ok) {
            keep(&mut kept, &line);
            let _ = tx.send(line);
        }
        kept
    });
    Stream { lines, reader }
}

/// Stdout keeps raw bytes, a newline after each line.
fn keep_bytes(bytes: &mut Vec<u8>, line: &str) {
    bytes.extend_from_slice(line.as_bytes());
    bytes.push(b'\n');
}

/// Stderr keeps its lines, for the failure message.
fn keep_lines(lines: &mut Vec<String>, line: &str) {
    lines.push(line.to_string());
}

/// Hand both streams' lines to `on_output` until both close, killing the
/// command once if `deadline` passes first. Returns whether it was killed.
fn relay(
    out: &Receiver<String>,
    err: &Receiver<String>,
    on_output: &dyn Fn(&str),
    deadline: Option<Instant>,
    kill: &dyn Fn(),
) -> bool {
    let (mut out_open, mut err_open, mut killed) = (true, true, false);
    while out_open || err_open {
        out_open = out_open && relay_one(out, on_output);
        err_open = err_open && relay_one(err, on_output);
        if !killed && deadline.is_some_and(|d| Instant::now() >= d) {
            kill();
            killed = true;
        }
        if out_open || err_open {
            std::thread::sleep(IDLE_POLL);
        }
    }
    killed
}

/// Relay one ready line, if any. Returns false once the stream has closed.
fn relay_one(lines: &Receiver<String>, on_output: &dyn Fn(&str)) -> bool {
    match lines.try_recv() {
        Ok(line) => {
            on_output(&line);
            true
        }
        Err(TryRecvError::Empty) => true,
        Err(TryRecvError::Disconnected) => false,
    }
}

/// A reader thread's result, or an error naming the stream if it panicked.
fn join<T>(reader: JoinHandle<T>, name: &str) -> std::io::Result<T> {
    reader
        .join()
        .map_err(|_| std::io::Error::other(format!("{name} reader thread panicked")))
}
