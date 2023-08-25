use std::io;
use tracing_appender::rolling;
use tracing_subscriber::{self, fmt::writer::MakeWriterExt};

pub fn initialize_logger() {
    let warn_file = rolling::daily("./logs", "log");
    let all_files = warn_file.and(io::stdout);

    tracing_subscriber::fmt()
        .json()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(all_files)
        .init();
}
