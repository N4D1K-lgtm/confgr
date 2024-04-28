pub mod core {
    pub use confgr_core::*;
}

pub mod derive {
    pub use confgr_derive::*;
}

pub mod config {
    pub use config::{Config, File};
}

pub mod prelude {
    pub use crate::core::{Empty, FromEnv, FromFile, Load, Merge};
    pub use crate::derive::Config;
}
