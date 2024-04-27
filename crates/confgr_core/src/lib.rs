pub trait Merge {
    fn merge(self, other: Self) -> Self;
}

pub trait Empty {
    fn empty() -> Self;
}
pub trait FromEnv {
    fn from_env() -> Self;
}

pub trait FromFile: Sized {
    fn from_file() -> Result<Self, String>;
}

pub trait Config {
    type Layer: Default + FromEnv + Merge + FromFile;
    fn config() -> Self;
}
