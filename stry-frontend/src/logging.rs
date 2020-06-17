use {std::fmt, warp::log::Info};

pub fn log(info: Info) {
    tracing::info!(
        target: "stry",
        "{} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
        OptFmt(info.remote_addr()),
        info.method(),
        info.path(),
        info.version(),
        info.status().as_u16(),
        OptFmt(info.referer()),
        OptFmt(info.user_agent()),
        info.elapsed(),
    );
}

// This is just warps private OptFmt
struct OptFmt<T>(Option<T>);

impl<T: fmt::Display> fmt::Display for OptFmt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref t) = self.0 {
            fmt::Display::fmt(t, f)
        } else {
            f.write_str("-")
        }
    }
}
