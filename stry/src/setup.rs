use {
    std::{fs::File, io::BufWriter, sync::Arc},
    stry_config::{Config, LogLevel, LoggingOutput},
    tracing::Level,
    tracing_appender::non_blocking::WorkerGuard,
    tracing_flame::FlameLayer,
    tracing_flame::FlushGuard,
    tracing_subscriber::{
        filter::LevelFilter,
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        Registry,
    },
};

#[allow(clippy::type_complexity)]
pub fn logger(
    cfg: Arc<Config>,
) -> anyhow::Result<(
    Option<WorkerGuard>,
    Option<FlushGuard<BufWriter<File>>>,
    impl tracing::Subscriber + Send + Sync + 'static,
)> {
    let mut stdout = (None, None);
    let mut file = (None, None, None);

    let mut setup_stdout = |json: bool| {
        if json {
            let layer = fmt::Layer::default()
                .with_ansi(cfg.logging.ansi)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE)
                .json();

            stdout.0 = Some(layer);
        } else {
            let layer = fmt::Layer::default()
                .with_ansi(cfg.logging.ansi)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE);

            stdout.1 = Some(layer);
        }
    };

    let mut setup_file = |directory: &str, json: bool, prefix: &str| -> anyhow::Result<()> {
        let file_appender = tracing_appender::rolling::daily(&directory, prefix);
        let (non_blocking, appender_guard) = tracing_appender::non_blocking(file_appender);

        if json {
            let layer = fmt::Layer::default()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE);

            file.0 = Some(layer);
        } else {
            let layer = fmt::Layer::default()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_thread_ids(cfg.logging.thread_ids)
                .with_thread_names(cfg.logging.thread_names)
                .with_span_events(FmtSpan::CLOSE);

            file.1 = Some(layer);
        }

        file.2 = Some(appender_guard);

        Ok(())
    };

    match &cfg.logging.out {
        LoggingOutput::Both {
            directory,
            json,
            prefix,
        } => {
            setup_stdout(*json);
            setup_file(directory.as_str(), *json, prefix.as_str())?;
        }
        LoggingOutput::File {
            directory,
            json,
            prefix,
        } => {
            setup_file(directory.as_str(), *json, prefix.as_str())?;
        }
        LoggingOutput::StdOut { json } => {
            setup_stdout(*json);
        }
    };

    let flame = if let Some(file) = cfg.logging.flame.as_ref() {
        let (layer, _guard) = FlameLayer::with_file(file)?;

        (Some(layer), Some(_guard))
    } else {
        (None, None)
    };

    let reg = Registry::default()
        .with(stdout.0)
        .with(stdout.1)
        .with(file.0)
        .with(file.1)
        .with(flame.0)
        // .with(tracing_tracy::TracyLayer::new())
        .with(LevelFilter::from(match cfg.logging.level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }));

    Ok((file.2, flame.1, reg))
}
