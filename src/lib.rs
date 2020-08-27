pub mod backend;
pub mod frontend;
pub mod models;
pub mod workers;

pub mod config;
pub mod search;
pub mod version;

use {
    crate::{
        backend::{Backend, DataBackend},
        config::{Config, LogLevel},
    },
    anyhow::Context,
    fenn::VecExt,
    std::{future::Future, pin::Pin, sync::Arc},
    tokio::{runtime::Builder, sync::broadcast},
    tracing::{Level, Subscriber},
    tracing_flame::FlameLayer,
    tracing_subscriber::{Layer,
        filter::LevelFilter,
        fmt::{self, format::FmtSpan},
        layer::SubscriberExt,
        Registry,
    },
};

pub type Boxed = Pin<Box<dyn Future<Output = anyhow::Result<()>>>>;

pub fn setup(cfg: Config) -> anyhow::Result<()> {
    let cfg = Arc::new(cfg);

    let (normal, json) = if cfg.logging.json {
        let layer = fmt::Layer::default()
            .with_ansi(cfg.logging.ansi)
            .with_thread_ids(cfg.logging.thread_ids)
            .with_thread_names(cfg.logging.thread_names)
            .with_span_events(FmtSpan::CLOSE)
            .json();

        (OptionLayer::None, OptionLayer::Some(layer))
    } else {
        let layer = fmt::Layer::default()
            .with_ansi(cfg.logging.ansi)
            .with_thread_ids(cfg.logging.thread_ids)
            .with_thread_names(cfg.logging.thread_names)
            .with_span_events(FmtSpan::CLOSE);

        (OptionLayer::Some(layer), OptionLayer::None)
    };

    let (file_normal, file_json, _appender_guard) =
        if let Some(directory) = cfg.logging.directory.as_ref() {
            let file_appender = tracing_appender::rolling::daily(&directory, &cfg.logging.prefix);
            let (non_blocking, appender_guard) = tracing_appender::non_blocking(file_appender);

            if cfg.logging.json {
                let layer = fmt::Layer::default()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_thread_ids(cfg.logging.thread_ids)
                    .with_thread_names(cfg.logging.thread_names)
                    .with_span_events(FmtSpan::CLOSE);

                (OptionLayer::Some(layer), OptionLayer::None, OptionLayer::Some(appender_guard))
            } else {
                let layer = fmt::Layer::default()
                    .with_writer(non_blocking)
                    .with_ansi(false)
                    .with_thread_ids(cfg.logging.thread_ids)
                    .with_thread_names(cfg.logging.thread_names)
                    .with_span_events(FmtSpan::CLOSE);
                (OptionLayer::None, OptionLayer::Some(layer), OptionLayer::Some(appender_guard))
            }
        } else {
            (OptionLayer::None, OptionLayer::None, OptionLayer::None)
        };

    let (flame, _flame_guard) = if let Some(file) = cfg.logging.flame.as_ref() {
        let (layer, _guard) = FlameLayer::with_file(file)?;

        (OptionLayer::Some(layer), OptionLayer::Some(_guard))
    } else {
        (OptionLayer::None, OptionLayer::None)
    };

    // TODO: Get JSON output working
    let reg = Registry::default()
        .with(normal)
        .with(json)
        .with(file_normal)
        .with(file_json)
        .with(flame)
        .with(LevelFilter::from(match cfg.logging.level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }));

    tracing::subscriber::set_global_default(reg)
        .context("Failed to set Tracing global subscriber")?;

    tracing_log::LogTracer::init().context("Failed to set Tracing as global Log drain")?;

    let mut builder = Builder::new();

    builder.threaded_scheduler();

    if let Some(count) = &cfg.executor.core_threads {
        builder.core_threads(*count);
    }

    if let Some(count) = &cfg.executor.max_threads {
        builder.max_threads(*count);
    }

    let mut rt = builder
        .enable_all()
        .build()
        .context("Unable to build Tokio runtime")?;

    rt.block_on(run(cfg))?;

    tracing::info!("Thank you. I'll say goodbye soon.. Though its the end of the would. Don't blame yourself now.. I'll be okay");

    Ok(())
}

async fn run(cfg: Arc<Config>) -> anyhow::Result<()> {
    let (tx, frontend_rx) = broadcast::channel::<()>(2);
    let download_rx = tx.subscribe();

    ctrlc::set_handler(move || {
        if tx.send(()).is_err() {
            tracing::error!("Unable to send shutdown signal");
        }
    })?;

    let version_info = Arc::new(
        backend::version()
            .appended(&mut workers::version())
            .sorted(),
    );

    let backend = DataBackend::init(cfg.database.typ, cfg.database.storage.clone(), version_info)
        .await
        .context("Unable to create backend instance")?;

    let download_handle = tokio::spawn(workers::start(cfg.clone(), download_rx, backend.clone()));
    let frontend_handle = tokio::spawn(frontend::start(cfg.clone(), frontend_rx, backend, true));

    download_handle
        .await
        .context("Unable to join download process with main")?;
    frontend_handle
        .await
        .context("Unable to join frontend process with main")?;

    Ok(())
}

#[derive(Debug)]
enum OptionLayer<T> {
    None,
    Some(T),
}

impl<L, S> Layer<S> for OptionLayer<L>
where
    L: Layer<S>,
    S: Subscriber,
{
    #[inline]
    fn new_span(&self, attrs: &tracing::span::Attributes<'_>, id: &tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.new_span(attrs, id, ctx)
        }
    }

    #[inline]
    fn register_callsite(&self, metadata: &'static tracing::metadata::Metadata<'static>) -> tracing::subscriber::Interest {
        match self {
            OptionLayer::Some(ref inner) => inner.register_callsite(metadata),
            OptionLayer::None => tracing::subscriber::Interest::always(),
        }
    }

    #[inline]
    fn enabled(&self, metadata: &tracing::metadata::Metadata<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) -> bool {
        match self {
            OptionLayer::Some(ref inner) => inner.enabled(metadata, ctx),
            OptionLayer::None => true,
        }
    }

    #[inline]
    fn max_level_hint(&self) -> Option<LevelFilter> {
        match self {
            OptionLayer::Some(ref inner) => inner.max_level_hint(),
            OptionLayer::None => None,
        }
    }

    #[inline]
    fn on_record(&self, span: &tracing::span::Id, values: &tracing::span::Record<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_record(span, values, ctx);
        }
    }

    #[inline]
    fn on_follows_from(&self, span: &tracing::span::Id, follows: &tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_follows_from(span, follows, ctx);
        }
    }

    #[inline]
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_event(event, ctx);
        }
    }

    #[inline]
    fn on_enter(&self, id: &tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_enter(id, ctx);
        }
    }

    #[inline]
    fn on_exit(&self, id: &tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_exit(id, ctx);
        }
    }

    #[inline]
    fn on_close(&self, id: tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_close(id, ctx);
        }
    }

    #[inline]
    fn on_id_change(&self, old: &tracing::span::Id, new: &tracing::span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_id_change(old, new, ctx)
        }
    }

    #[doc(hidden)]
    #[inline]
    unsafe fn downcast_raw(&self, id: std::any::TypeId) -> Option<*const ()> {
        if id == std::any::TypeId::of::<Self>() {
            Some(self as *const _ as *const ())
        } else {
            match *self {
                OptionLayer::Some(ref inner) => inner.downcast_raw(id),
                OptionLayer::None => None,
            }
        }
    }
}
