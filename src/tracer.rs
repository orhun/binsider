use std::io::Cursor;
use std::sync::mpsc;
use std::thread;

use console::Style;
use lurk_cli::args::Args;
use lurk_cli::style::StyleConfig;
use lurk_cli::Tracer;
use nix::sys::wait::{waitpid, WaitPidFlag};

use crate::error::{Error, Result};
use crate::tui::event::Event;

use nix::unistd::{fork, ForkResult};

/// Trace system calls and signals.
pub fn trace_syscalls(command: &str, event_sender: mpsc::Sender<Event>) {
    let event_sender = event_sender.clone();
    let command = command.to_string();
    thread::spawn(move || {
        let run_tracer = || -> Result<()> {
            let pid = match unsafe { fork() } {
                Ok(ForkResult::Child) => {
                    return lurk_cli::run_tracee(&[command], &[], &None)
                        .map_err(|e| Error::TraceError(e.to_string()))
                }
                Ok(ForkResult::Parent { child }) => child,
                Err(err) => return Err(Error::TraceError(format!("fork() failed: {err}"))),
            };
            let mut output = Vec::new();
            Tracer::new(
                pid,
                Args::default(),
                Box::new(Cursor::new(&mut output)),
                StyleConfig {
                    pid: Style::new().cyan(),
                    syscall: Style::new().white().bold(),
                    success: Style::new().green(),
                    error: Style::new().red(),
                    result: Style::new().yellow(),
                    use_colors: true,
                },
            )
            .map_err(|e| Error::TraceError(e.to_string()))?
            .run_tracer()
            .map_err(|e| Error::TraceError(e.to_string()))?;
            let _ = waitpid(pid, Some(WaitPidFlag::WNOHANG));
            event_sender
                .send(Event::TraceResult(Ok(output)))
                .map_err(|e| Error::ChannelSendError(e.to_string()))?;
            Ok(())
        };
        if let Err(e) = run_tracer() {
            event_sender
                .send(Event::TraceResult(Err(e)))
                .expect("failed to send the trace result")
        }
    });
}
