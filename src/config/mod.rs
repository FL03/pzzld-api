/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub(crate) use self::utils::*;
pub use self::{context::*, kinds::prelude::*, settings::*};

pub(crate) mod context;
pub(crate) mod settings;

pub(crate) type ArcCtx = std::sync::Arc<Context>;

pub mod kinds {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod mode;
    pub mod server;

    pub(crate) mod prelude {
        pub use super::mode::*;
        pub use super::server::*;
    }
}

pub(crate) mod prelude {
    pub use super::context::Context;
    pub use super::kinds::prelude::*;
    pub use super::settings::Settings;
}

pub(crate) mod utils {
    use std::path::PathBuf;

    /// Type alias for [config::File]
    pub(crate) type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;
    /// Type alias for a collection of [crate::ConfigFile]
    pub(crate) type ConfigFileVec = Vec<ConfigFile<config::FileSourceFile, config::FileFormat>>;

    /// A generic function wrapper extending glob::glob
    fn gather<F, T>(pattern: &str, f: F) -> Vec<T>
    where
        F: Fn(PathBuf) -> T,
    {
        glob::glob(pattern)
            .expect("Failed to collect files")
            .filter_map(|r| r.ok().map(|p| f(p)))
            .collect()
    }

    /// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
    pub fn collect_configurations(pattern: &str, required: bool) -> ConfigFileVec {
        gather(pattern, |p| ConfigFile::from(p).required(required))
    }

    pub(crate) fn init_tracing(level: tracing::Level) {
        use tracing_subscriber::fmt::time;

        tracing_subscriber::fmt()
            .compact()
            .with_ansi(true)
            .with_max_level(level)
            .with_target(false)
            .with_timer(time::uptime())
            .init();
    }
}
