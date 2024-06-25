/*
    Appellation: logger <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct LoggerConfig {
    pub level: LogLevel
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
    #[default]
    Off
}

impl LoggerConfig {
    pub fn init_tracing(&self) {
        let level = self.level.as_tracing();
        if let Some(level) = level {
            crate::config::init_tracing(level)
        }
    }
}

impl LogLevel {
    pub fn debug() -> Self {
        Self::Debug
    }

    pub fn info() -> Self {
        Self::Info
    }

    pub fn warn() -> Self {
        Self::Warn
    }

    pub fn error() -> Self {
        Self::Error
    }

    pub fn off() -> Self {
        Self::Off
    }

    pub fn as_tracing(&self) -> Option<tracing::Level> {
        use tracing::Level;

        match self {
            Self::Debug => Some(Level::DEBUG),
            Self::Error => Some(Level::ERROR),
            Self::Info => Some(Level::INFO),
            Self::Trace => Some(Level::TRACE),
            Self::Warn => Some(Level::WARN),
            Self::Off => None
        }
    }

}


/*
    ************* Implementations *************
*/
impl core::fmt::Display for LoggerConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

unsafe impl Send for LoggerConfig {}

unsafe impl Sync for LoggerConfig {}

mod impl_level {
    use super::*;

    
    impl From<LogLevel> for config::Value {
        fn from(level: LogLevel) -> Self {
            level.to_string().into()
        }
    }

    impl From<tracing::Level> for LogLevel {
        fn from(level: tracing::Level) -> Self {
            match level {
                tracing::Level::DEBUG => Self::Debug,
                tracing::Level::ERROR => Self::Error,
                tracing::Level::TRACE => Self::Trace,
                tracing::Level::INFO => Self::Info,
                tracing::Level::WARN => Self::Warn,
            }
        }
    }


    unsafe impl Send for LogLevel {}

    unsafe impl Sync for LogLevel {}

}