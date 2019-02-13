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

#[derive(Debug)]
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
    expected_ok_status: Vec<Status>,
    expected_error_status: Vec<Status>,
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

        if self.args.len() > 0 {
            cmd = cmd + SPACE + self.args.clone().join(SPACE).as_str();
        }

        if self.body.is_some() {
            cmd = cmd + SPACE + self.body.unwrap().len().to_string().as_str() + LINE_BREAK;

            let utf8body = unsafe { String::from_utf8_unchecked(self.body.unwrap().to_vec()) };
            cmd.push_str(utf8body.as_str());
        }
        cmd.push_str(LINE_BREAK);

        cmd
    }

    pub fn expected_ok_status(&self) -> &Vec<Status> {
        &self.expected_ok_status
    }

    pub fn expected_error_status(&self) -> &Vec<Status> {
        &self.expected_error_status
    }

    // Construct commands
    pub fn put(body: &'a [u8], priority: u32, delay: Duration, ttr: Duration) -> Self {
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

    pub fn reserve(timeout: Option<Duration>) -> Self {
        Command::new(
            match timeout {
                None => CommandKind::Reserve,
                Some(_) => CommandKind::ReserveTimeout,
            },
            timeout
                .map(|t| vec![t.as_secs().to_string()])
                .unwrap_or(vec![]),
            None,
            vec![Status::Reserved],
            vec![Status::TimedOut, Status::DeadlineSoon],
        )
    }

    pub fn kick(bound: u32) -> Self {
        Command::new(
            CommandKind::Kick,
            vec![bound.to_string()],
            None,
            vec![Status::Kicked],
            vec![],
        )
    }

    pub fn kick_job(job_id: u64) -> Self {
        Command::new(
            CommandKind::JobKick,
            vec![job_id.to_string()],
            None,
            vec![Status::Kicked],
            vec![Status::NotFound],
        )
    }

    pub fn peek_job(job_id: u64) -> Self {
        Self::peek(CommandKind::PeekJob, vec![job_id.to_string()])
    }

    pub fn peek_ready() -> Self {
        Self::peek(CommandKind::PeekReady, vec![])
    }

    pub fn peek_delayed() -> Self {
        Self::peek(CommandKind::PeekDelayed, vec![])
    }

    pub fn peek_buried() -> Self {
        Self::peek(CommandKind::PeekBuried, vec![])
    }

    fn peek(kind: CommandKind, args: Vec<String>) -> Self {
        Command::new(
            kind,
            args,
            None,
            vec![Status::Found],
            vec![Status::NotFound],
        )
    }

    pub fn tubes() -> Self {
        Command::new(
            CommandKind::ListTubes,
            vec![],
            None,
            vec![Status::Ok],
            vec![],
        )
    }

    pub fn using() -> Self {
        Command::new(
            CommandKind::ListTubeUsed,
            vec![],
            None,
            vec![Status::Using],
            vec![],
        )
    }

    pub fn use_tube(name: &str) -> Self {
        Command::new(
            CommandKind::Use,
            vec![name.to_string()],
            None,
            vec![Status::Using],
            vec![],
        )
    }

    pub fn watch(name: &str) -> Self {
        Command::new(
            CommandKind::Watch,
            vec![name.to_string()],
            None,
            vec![Status::Watching],
            vec![],
        )
    }

    pub fn ignore(name: &str) -> Self {
        Command::new(
            CommandKind::Ignore,
            vec![name.to_string()],
            None,
            vec![Status::Watching],
            vec![Status::NotIgnored],
        )
    }

    pub fn stats() -> Self {
        Command::new(CommandKind::Stats, vec![], None, vec![Status::Ok], vec![])
    }

    pub fn stats_tube(name: &str) -> Self {
        Command::new(
            CommandKind::StatsTube,
            vec![name.to_string()],
            None,
            vec![Status::Ok],
            vec![Status::NotFound],
        )
    }

    pub fn pause_tube(name: &str) -> Self {
        Command::new(
            CommandKind::PauseTube,
            vec![name.to_string()],
            None,
            vec![Status::Ok],
            vec![Status::NotFound],
        )
    }

    pub fn delete(job_id: u64) -> Self {
        Command::new(
            CommandKind::Delete,
            vec![job_id.to_string()],
            None,
            vec![Status::Deleted],
            vec![Status::NotFound],
        )
    }

    pub fn release(job_id: u64, priority: u32, delay: Duration) -> Self {
        Command::new(
            CommandKind::Release,
            vec![priority.to_string(), delay.as_secs().to_string()],
            None,
            vec![Status::Released, Status::Buried],
            vec![Status::NotFound],
        )
    }

    pub fn bury(job_id: u64, priority: u32) -> Self {
        Command::new(
            CommandKind::Bury,
            vec![job_id.to_string(), priority.to_string()],
            None,
            vec![Status::Buried],
            vec![Status::NotFound],
        )
    }

    pub fn touch(job_id: u64) -> Self {
        Command::new(
            CommandKind::Touch,
            vec![job_id.to_string()],
            None,
            vec![Status::Ok],
            vec![Status::NotFound],
        )
    }

    pub fn stats_job(job_id: u64) -> Self {
        Command::new(
            CommandKind::JobStats,
            vec![job_id.to_string()],
            None,
            vec![Status::Ok],
            vec![Status::NotFound],
        )
    }
}
