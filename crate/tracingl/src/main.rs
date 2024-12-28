use tracing::{info, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

fn main() {
    let guard = init_tracing();
    info!("Hello, world!");
    info!("Hello, world!2");
    warn!("warning 1");
}

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("./logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 4. 构造控制台 Layer
    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true);

    // 5. 构造文件输出 Layer
    let file_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking);

    Registry::default()
        .with(console_layer)
        .with(file_layer)
        .init();
    _guard
}
