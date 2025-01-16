mod file;
mod io_rate_limiter;

mod settings {
    use crate::io_rate_limiter::IoType;

    // FIXME:
    pub fn get_io_type() -> IoType {
        return IoType::Flush;
    }
}
