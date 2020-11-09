#[doc(hidden)]
#[macro_export]
macro_rules! opt_try {
    ($opt:ident) => {
        match $opt {
            Some(item) => item,
            None => return Ok(None),
        }
    };
}
