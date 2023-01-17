use core::fmt;

use log::{Level, LevelFilter, Log, Metadata, Record};
use spin::Mutex;

lazy_static! {
    static ref LOG_LOCK: Mutex<()> = Mutex::new(());
}

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();

    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

/// Add escape sequence to print with color in Linux console
macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn print_with_color(args: fmt::Arguments, color_code: u8) {
    let _guard = LOG_LOCK.lock();
    crate::console::print(with_color!(args, color_code));
}

struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        print_with_color(
            format_args!("[{:>5}] {}\n", record.level(), record.args()),
            color_code_from_log_level(record.level()),
        );
    }
    fn flush(&self) {}
}

fn color_code_from_log_level(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}