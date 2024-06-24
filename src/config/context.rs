/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Settings;
use std::sync::Arc;

pub trait Ctx: Send + Sync {
    type Config;
}

pub trait AxumCtx: Ctx + Sized {
    fn into_ext_shared(self: Arc<Self>) -> axum::Extension<Arc<Self>> {
        axum::Extension(self)
    }

    fn into_ext(self) -> axum::Extension<Self> {
        axum::Extension(self)
    }
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Context {
    pub(crate) settings: Settings,
}

impl Context {
    pub fn from_config(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn try_to_build() -> Result<Self, config::ConfigError> {
        Settings::build().map(Self::from_config)
    }

    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn init_tracing(&self) {
        let level = self.settings().mode().as_tracing();
        super::init_tracing(level)
    }

    pub fn into_ext_shared(self: Arc<Self>) -> axum::Extension<Arc<Self>> {
        axum::Extension(self)
    }

    pub fn into_ext(self) -> axum::Extension<Self> {
        axum::Extension(self)
    }

    pub fn with_tracing(self) -> Self {
        self.init_tracing();
        self
    }
}

/*
 ************* Implementations *************
*/
impl AsRef<Settings> for Context {
    fn as_ref(&self) -> &Settings {
        &self.settings
    }
}

impl AsMut<Settings> for Context {
    fn as_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
}

impl core::borrow::Borrow<Settings> for Context {
    fn borrow(&self) -> &Settings {
        self.settings.borrow()
    }
}

impl core::borrow::BorrowMut<Settings> for Context {
    fn borrow_mut(&mut self) -> &mut Settings {
        self.settings.borrow_mut()
    }
}

impl Default for Context {
    fn default() -> Self {
        let settings = Settings::default();
        Self { settings }
    }
}

impl core::ops::Deref for Context {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl core::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

impl From<Settings> for Context {
    fn from(settings: Settings) -> Self {
        Self { settings }
    }
}

unsafe impl core::marker::Send for Context {}

unsafe impl core::marker::Sync for Context {}
