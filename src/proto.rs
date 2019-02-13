use std::string::ToString;
use std::time::Duration;

use self::cmdkind::CommandKind;
use self::status::Status;

mod cmdkind;
mod status;

#[derive(Debug)]
pub struct Command<'a> {
    kind: CommandKind,
    args: Option<Vec<String>>,
    body: Option<&'a [u8]>,
    expected_ok_status: Vec<Status>,
    expected_error_status: Vec<Status>,
}

impl<'a> Command<'a> {
    fn new(
        kind: CommandKind,
        args: Option<Vec<String>>,
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

        if self.args.is_some() {
            cmd = cmd + SPACE + self.args.clone().unwrap().join(SPACE).as_str();
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
        Self::new(
            CommandKind::Put,
            Some(vec![
                priority.to_string(),
                delay.as_secs().to_string(),
                ttr.as_secs().to_string(),
            ]),
            Some(body),
            vec![Status::Inserted],
            vec![Status::JobTooBig, Status::Buried, Status::Draining],
        )
    }

    pub fn reserve(timeout: Option<Duration>) -> Self {
        Self::new(
            match timeout {
                None => CommandKind::Reserve,
                Some(_) => CommandKind::ReserveTimeout,
            },
            timeout.map(|t|vec![t.as_secs().to_string()]),
            None,
            vec![Status::Reserved],
            vec![Status::TimedOut, Status::DeadlineSoon],
        )
    }
}
