use {
    std::any::TypeId,
    tracing::{
        metadata::Metadata,
        span::{Attributes, Id, Record},
        subscriber::Interest,
        Event, Subscriber,
    },
    tracing_subscriber::{filter::LevelFilter, layer::Context, Layer},
};

#[derive(Debug)]
pub enum OptionLayer<T> {
    None,
    Some(T),
}

impl<L, S> Layer<S> for OptionLayer<L>
where
    L: Layer<S>,
    S: Subscriber,
{
    #[inline]
    fn new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.new_span(attrs, id, ctx)
        }
    }

    #[inline]
    fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
        match self {
            OptionLayer::Some(ref inner) => inner.register_callsite(metadata),
            OptionLayer::None => Interest::always(),
        }
    }

    #[inline]
    fn enabled(&self, metadata: &Metadata<'_>, ctx: Context<'_, S>) -> bool {
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
    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_record(span, values, ctx);
        }
    }

    #[inline]
    fn on_follows_from(&self, span: &Id, follows: &Id, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_follows_from(span, follows, ctx);
        }
    }

    #[inline]
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_event(event, ctx);
        }
    }

    #[inline]
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_enter(id, ctx);
        }
    }

    #[inline]
    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_exit(id, ctx);
        }
    }

    #[inline]
    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_close(id, ctx);
        }
    }

    #[inline]
    fn on_id_change(&self, old: &Id, new: &Id, ctx: Context<'_, S>) {
        if let OptionLayer::Some(ref inner) = self {
            inner.on_id_change(old, new, ctx)
        }
    }

    #[doc(hidden)]
    #[inline]
    unsafe fn downcast_raw(&self, id: TypeId) -> Option<*const ()> {
        if id == TypeId::of::<Self>() {
            Some(self as *const _ as *const ())
        } else {
            match *self {
                OptionLayer::Some(ref inner) => inner.downcast_raw(id),
                OptionLayer::None => None,
            }
        }
    }
}
