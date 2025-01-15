use chrono::{DateTime, Local};
use log::{Level, LevelFilter, Log, Record};
use tokio::sync::mpsc::UnboundedSender;

pub fn init(
    filter: LevelFilter,
    sender: UnboundedSender<LoggerRecord>,
) -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(ChannelLogger::new(sender))).map(|()| log::set_max_level(filter))
}

#[derive(Clone, Debug)]
pub struct LoggerRecord {
    pub message: String,
    pub level: Level,
    pub time: DateTime<Local>,
}

impl<'a> From<&Record<'a>> for LoggerRecord {
    fn from(record: &Record<'a>) -> Self {
        Self {
            message: record.args().to_string(),
            level: record.level(),
            time: Local::now(),
        }
    }
}

struct ChannelLogger {
    sender: UnboundedSender<LoggerRecord>,
}

impl ChannelLogger {
    fn new(sender: UnboundedSender<LoggerRecord>) -> ChannelLogger {
        Self { sender }
    }
}

impl Log for ChannelLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let _ = self.sender.send(LoggerRecord::from(record));
        }
    }

    fn flush(&self) {}
}
