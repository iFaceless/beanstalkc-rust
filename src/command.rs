use std::str::FromStr;
use std::string::ToString;
use std::time::Duration;

use crate::error::BeanstalkcError;

#[derive(Debug)]
pub enum CommandKind {
    Put,
    PeekJob,
    PeekReady,
    PeekDelayed,
    PeekBuried,
    Reserve,
    ReserveTimeout,
    Delete,
    Release,
    Bury,
    Kick,
    JobKick,
    Touch,
    Stats,
    JobStats,
    Use,
    Watch,
    Ignore,
    ListTubes,
    ListTubeUsed,
    ListTubesWatched,
    StatsTube,
    Quit,
    PauseTube,
}

impl ToString for CommandKind {
    fn to_string(&self) -> String {
        let cmd = match *self {
            CommandKind::Put => "put",
            CommandKind::PeekJob => "peek",
            CommandKind::PeekReady => "peek-ready",
            CommandKind::PeekDelayed => "peek-delayed",
            CommandKind::PeekBuried => "peek-buried",
            CommandKind::Reserve => "reserve",
            CommandKind::ReserveTimeout => "reserve-with-timeout",
            CommandKind::Delete => "delete",
            CommandKind::Release => "release",
            CommandKind::Bury => "bury",
            CommandKind::Kick => "kick",
            CommandKind::JobKick => "kick-job",
            CommandKind::Touch => "touch",
            CommandKind::Stats => "stats",
            CommandKind::JobStats => "stats-job",
            CommandKind::Use => "use",
            CommandKind::Watch => "watch",
            CommandKind::Ignore => "ignore",
            CommandKind::ListTubes => "list-tubes",
            CommandKind::ListTubeUsed => "list-tube-used",
            CommandKind::ListTubesWatched => "list-tubes-watched",
            CommandKind::StatsTube => "stats-tube",
            CommandKind::Quit => "quit",
            CommandKind::PauseTube => "pause-tube",
        };
        cmd.to_string()
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Ok,
    Found,
    NotFound,
    Reserved,
    DeadlineSoon,
    TimedOut,
    Deleted,
    Released,
    Buried,
    Kicked,
    Using,
    Watching,
    Touched,
    Inserted,
    NotIgnored,
    OutOfMemory,
    InternalError,
    Draining,
    BadFormat,
    UnknownCommand,
    ExpectedCRLF,
    JobTooBig,
    Paused,
}

impl FromStr for Status {
    type Err = BeanstalkcError;

    fn from_str(s: &str) -> Result<Self, BeanstalkcError> {
        let s = match s {
            "OK" => Status::Ok,
            "FOUND" => Status::Found,
            "NOT_FOUND" => Status::NotFound,
            "RESERVED" => Status::Reserved,
            "DEADLINE_SOON" => Status::DeadlineSoon,
            "TIMED_OUT" => Status::TimedOut,
            "DELETED" => Status::Deleted,
            "RELEASED" => Status::Released,
            "BURIED" => Status::Buried,
            "KICKED" => Status::Kicked,
            "USING" => Status::Using,
            "WATCHING" => Status::Watching,
            "TOUCHED" => Status::Touched,
            "INSERTED" => Status::Inserted,
            "NOT_IGNORED" => Status::NotIgnored,
            "OUT_OF_MEMORY" => Status::OutOfMemory,
            "INTERNAL_ERROR" => Status::InternalError,
            "DRAINING" => Status::Draining,
            "BAD_FORMAT" => Status::BadFormat,
            "UNKNOWN_COMMAND" => Status::UnknownCommand,
            "EXPECTED_CRLF" => Status::ExpectedCRLF,
            "JOB_TOO_BIG" => Status::JobTooBig,
            "PAUSED" => Status::Paused,
            _ => {
                return Err(BeanstalkcError::CommandFailed(s.to_string()));
            }
        };
        Ok(s)
    }
}

#[derive(Debug)]
pub struct Command<'a> {
    kind: CommandKind,
    args: Vec<String>,
    body: Option<&'a [u8]>,
    pub expected_ok_status: Vec<Status>,
    pub expected_error_status: Vec<Status>,
}

impl<'a> Command<'a> {
    fn new(
        kind: CommandKind,
        args: Vec<String>,
        body: Option<&'a [u8]>,
        exp_ok_status: Vec<Status>,
        exp_error_status: Vec<Status>,
    ) -> Self {
        Command {
            kind,
            args,
            body,
            expected_ok_status: exp_ok_status,
            expected_error_status: exp_error_status,
        }
    }

    pub fn build(&self) -> String {
        const SPACE: &str = " ";
        const LINE_BREAK: &str = "\r\n";

        let mut cmd = self.kind.to_string();

        if !self.args.is_empty() {
            cmd = cmd + SPACE + self.args.clone().join(SPACE).as_str();
        }

        if let Some(body) = self.body {
            cmd = cmd + SPACE + body.len().to_string().as_str() + LINE_BREAK;

            let utf8body = String::from_utf8_lossy(body);
            cmd.push_str(&utf8body);
        }
        cmd.push_str(LINE_BREAK);

        cmd
    }
}

// Construct commands
pub fn put(body: &[u8], priority: u32, delay: Duration, ttr: Duration) -> Command {
    Command::new(
        CommandKind::Put,
        vec![
            priority.to_string(),
            delay.as_secs().to_string(),
            ttr.as_secs().to_string(),
        ],
        Some(body),
        vec![Status::Inserted],
        vec![Status::JobTooBig, Status::Buried, Status::Draining],
    )
}

pub fn reserve<'a>(timeout: Option<Duration>) -> Command<'a> {
    Command::new(
        match timeout {
            None => CommandKind::Reserve,
            Some(_) => CommandKind::ReserveTimeout,
        },
        timeout
            .map(|t| vec![t.as_secs().to_string()])
            .unwrap_or_else(|| vec![]),
        None,
        vec![Status::Reserved],
        vec![Status::TimedOut, Status::DeadlineSoon],
    )
}

pub fn kick<'a>(bound: u32) -> Command<'a> {
    Command::new(
        CommandKind::Kick,
        vec![bound.to_string()],
        None,
        vec![Status::Kicked],
        vec![],
    )
}

pub fn kick_job<'a>(job_id: u64) -> Command<'a> {
    Command::new(
        CommandKind::JobKick,
        vec![job_id.to_string()],
        None,
        vec![Status::Kicked],
        vec![Status::NotFound],
    )
}

pub fn peek_job<'a>(job_id: u64) -> Command<'a> {
    peek(CommandKind::PeekJob, vec![job_id.to_string()])
}

pub fn peek_ready<'a>() -> Command<'a> {
    peek(CommandKind::PeekReady, vec![])
}

pub fn peek_delayed<'a>() -> Command<'a> {
    peek(CommandKind::PeekDelayed, vec![])
}

pub fn peek_buried<'a>() -> Command<'a> {
    peek(CommandKind::PeekBuried, vec![])
}

fn peek<'a>(kind: CommandKind, args: Vec<String>) -> Command<'a> {
    Command::new(
        kind,
        args,
        None,
        vec![Status::Found],
        vec![Status::NotFound],
    )
}

pub fn tubes<'a>() -> Command<'a> {
    Command::new(
        CommandKind::ListTubes,
        vec![],
        None,
        vec![Status::Ok],
        vec![],
    )
}

pub fn using<'a>() -> Command<'a> {
    Command::new(
        CommandKind::ListTubeUsed,
        vec![],
        None,
        vec![Status::Using],
        vec![],
    )
}

pub fn use_tube<'a>(name: &str) -> Command<'a> {
    Command::new(
        CommandKind::Use,
        vec![name.to_string()],
        None,
        vec![Status::Using],
        vec![],
    )
}

pub fn watching<'a>() -> Command<'a> {
    Command::new(
        CommandKind::ListTubesWatched,
        vec![],
        None,
        vec![Status::Ok],
        vec![],
    )
}

pub fn watch<'a>(name: &str) -> Command<'a> {
    Command::new(
        CommandKind::Watch,
        vec![name.to_string()],
        None,
        vec![Status::Watching],
        vec![],
    )
}

pub fn ignore<'a>(name: &str) -> Command<'a> {
    Command::new(
        CommandKind::Ignore,
        vec![name.to_string()],
        None,
        vec![Status::Watching],
        vec![Status::NotIgnored],
    )
}

pub fn stats<'a>() -> Command<'a> {
    Command::new(CommandKind::Stats, vec![], None, vec![Status::Ok], vec![])
}

pub fn stats_tube<'a>(name: &str) -> Command<'a> {
    Command::new(
        CommandKind::StatsTube,
        vec![name.to_string()],
        None,
        vec![Status::Ok],
        vec![Status::NotFound],
    )
}

pub fn pause_tube<'a>(name: &str, delay: Duration) -> Command<'a> {
    Command::new(
        CommandKind::PauseTube,
        vec![name.to_string(), delay.as_secs().to_string()],
        None,
        vec![Status::Paused],
        vec![Status::NotFound],
    )
}

pub fn delete<'a>(job_id: u64) -> Command<'a> {
    Command::new(
        CommandKind::Delete,
        vec![job_id.to_string()],
        None,
        vec![Status::Deleted],
        vec![Status::NotFound],
    )
}

pub fn release<'a>(job_id: u64, priority: u32, delay: Duration) -> Command<'a> {
    Command::new(
        CommandKind::Release,
        vec![
            job_id.to_string(),
            priority.to_string(),
            delay.as_secs().to_string(),
        ],
        None,
        vec![Status::Released, Status::Buried],
        vec![Status::NotFound],
    )
}

pub fn bury<'a>(job_id: u64, priority: u32) -> Command<'a> {
    Command::new(
        CommandKind::Bury,
        vec![job_id.to_string(), priority.to_string()],
        None,
        vec![Status::Buried],
        vec![Status::NotFound],
    )
}

pub fn touch<'a>(job_id: u64) -> Command<'a> {
    Command::new(
        CommandKind::Touch,
        vec![job_id.to_string()],
        None,
        vec![Status::Touched],
        vec![Status::NotFound],
    )
}

pub fn stats_job<'a>(job_id: u64) -> Command<'a> {
    Command::new(
        CommandKind::JobStats,
        vec![job_id.to_string()],
        None,
        vec![Status::Ok],
        vec![Status::NotFound],
    )
}

pub fn quit<'a>() -> Command<'a> {
    Command::new(CommandKind::Quit, vec![], None, vec![], vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_put() {
        let cmd = put(
            b"Rust",
            0,
            Duration::from_secs(10),
            Duration::from_secs(100),
        );
        assert_eq!(cmd.build().as_str(), "put 0 10 100 4\r\nRust\r\n")
    }

    #[test]
    fn test_reserve() {
        let cmd = reserve(None);
        assert_eq!(cmd.build().as_str(), "reserve\r\n");

        let cmd = reserve(Some(Duration::from_secs(10)));
        assert_eq!(cmd.build().as_str(), "reserve-with-timeout 10\r\n")
    }

    #[test]
    fn test_kick() {
        let cmd = kick(100);
        assert_eq!(cmd.build().as_str(), "kick 100\r\n");
    }

    #[test]
    fn test_kick_job() {
        let cmd = kick_job(1);
        assert_eq!(cmd.build().as_str(), "kick-job 1\r\n");
    }

    #[test]
    fn test_peek_job() {
        let cmd = peek_job(1);
        assert_eq!(cmd.build().as_str(), "peek 1\r\n");
    }

    #[test]
    fn test_peek_ready() {
        let cmd = peek_ready();
        assert_eq!(cmd.build().as_str(), "peek-ready\r\n");
    }

    #[test]
    fn test_peek_buried() {
        let cmd = peek_buried();
        assert_eq!(cmd.build().as_str(), "peek-buried\r\n");
    }

    #[test]
    fn test_list_tubes() {
        let cmd = tubes();
        assert_eq!(cmd.build().as_str(), "list-tubes\r\n");
    }

    #[test]
    fn test_tube_used() {
        let cmd = using();
        assert_eq!(cmd.build().as_str(), "list-tube-used\r\n");
    }

    #[test]
    fn test_use_tube() {
        let cmd = use_tube("jobs");
        assert_eq!(cmd.build().as_str(), "use jobs\r\n");
    }

    #[test]
    fn test_tubes_watched() {
        let cmd = watching();
        assert_eq!(cmd.build().as_str(), "list-tubes-watched\r\n");
    }

    #[test]
    fn test_watch() {
        let cmd = watch("jobs");
        assert_eq!(cmd.build().as_str(), "watch jobs\r\n");
    }

    #[test]
    fn test_ignore() {
        let cmd = ignore("jobs");
        assert_eq!(cmd.build().as_str(), "ignore jobs\r\n");
    }

    #[test]
    fn test_stats_tube() {
        let cmd = stats_tube("jobs");
        assert_eq!(cmd.build().as_str(), "stats-tube jobs\r\n");
    }

    #[test]
    fn test_pause_tube() {
        let cmd = pause_tube("jobs", Duration::from_secs(1));
        assert_eq!(cmd.build().as_str(), "pause-tube jobs 1\r\n");
    }

    #[test]
    fn test_delete() {
        let cmd = delete(1);
        assert_eq!(cmd.build().as_str(), "delete 1\r\n");
    }

    #[test]
    fn test_release() {
        let cmd = release(100, 0, Duration::from_secs(100));
        assert_eq!(cmd.build().as_str(), "release 100 0 100\r\n");
    }

    #[test]
    fn test_bury() {
        let cmd = bury(100, 0);
        assert_eq!(cmd.build().as_str(), "bury 100 0\r\n");
    }

    #[test]
    fn test_touch() {
        let cmd = touch(100);
        assert_eq!(cmd.build().as_str(), "touch 100\r\n");
    }

    #[test]
    fn test_stats_job() {
        let cmd = stats_job(100);
        assert_eq!(cmd.build().as_str(), "stats-job 100\r\n");
    }

    #[test]
    fn test_quit() {
        let cmd = quit();
        assert_eq!(cmd.build().as_str(), "quit\r\n");
    }
}
