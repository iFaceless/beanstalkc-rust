// Commands
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
