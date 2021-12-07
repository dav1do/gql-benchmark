pub trait ResultLogger {
    #[track_caller]
    fn log_err(self) -> Self;

    #[track_caller]
    fn log_err_ctx(self, ctx: &str) -> Self;

    #[track_caller]
    fn log_warn(self) -> Self;

    #[track_caller]
    fn log_warn_ctx(self, ctx: &str) -> Self;
}

impl<T, E: std::error::Error> ResultLogger for Result<T, E> {
    #[track_caller]
    fn log_err(self) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                let location = core::panic::Location::caller();
                tracing::error!(error=%e, file=%location.file(), line=%location.line(), "{}", e);

                Err(e)
            }
        }
    }

    fn log_err_ctx(self, ctx: &str) -> Self {
        if ctx.is_empty() {
            self.log_err()
        } else {
            match self {
                Ok(t) => Ok(t),
                Err(e) => {
                    let location = core::panic::Location::caller();
                    tracing::error!(error=%e, file=%location.file(), line=%location.line(), "{}", ctx);

                    Err(e)
                }
            }
        }
    }

    fn log_warn(self) -> Self {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                let location = core::panic::Location::caller();
                tracing::warn!(error=%e, file=%location.file(), line=%location.line(), "{}", e);

                Err(e)
            }
        }
    }

    fn log_warn_ctx(self, ctx: &str) -> Self {
        if ctx.is_empty() {
            self.log_warn()
        } else {
            match self {
                Ok(t) => Ok(t),
                Err(e) => {
                    let location = core::panic::Location::caller();
                    tracing::warn!(error=%e, file=%location.file(), line=%location.line(), "{}", ctx);

                    Err(e)
                }
            }
        }
    }
}
