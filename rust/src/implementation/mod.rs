mod profiles;
mod passwordmaker;
mod pwm_macros;

pub use self::profiles::Profiles;
pub use self::passwordmaker::PasswordMaker;
pub use self::passwordmaker::Settings;

//------------------------------------------------------------------------------
// helper types that are used in multiple modules.

fn get_config_folder() -> Option<std::path::PathBuf> {
    dirs::config_dir()
        .map(|p| p.join("info.grois/harbour-passfish/"))
}

#[derive(Debug)]
enum LoadError {
    Xdg,
    Loading(std::io::Error),
    Parsing(toml::de::Error),
}
impl std::fmt::Display for LoadError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            LoadError::Xdg => { write!(f, "XDG config path not found") }
            LoadError::Loading(e) => { e.fmt(f) }
            LoadError::Parsing(e) => { e.fmt(f) }
        }
    }
}
impl std::error::Error for LoadError {}
impl From<std::io::Error> for LoadError {
    fn from(e : std::io::Error) -> Self {
        LoadError::Loading(e)
    }
}
impl From<toml::de::Error> for LoadError {
    fn from(e : toml::de::Error) -> Self {
        LoadError::Parsing(e)
    }
}


#[derive(Debug)]
enum StoreError {
    Xdg,
    Writing(std::io::Error),
    Serialization(toml::ser::Error)
}
impl std::fmt::Display for StoreError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            StoreError::Xdg => { write!(f, "XDC config path not found") }
            StoreError::Writing(e) => { e.fmt(f) }
            StoreError::Serialization(e) => { e.fmt(f) }
        }
    }
}
impl std::error::Error for StoreError {}
impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        StoreError::Writing(e)
    }
}
impl From<toml::ser::Error> for StoreError {
    fn from(e: toml::ser::Error) -> Self {
        StoreError::Serialization(e)
    }
}
