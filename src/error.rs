use thiserror::Error;

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("can't create dir")]
    CantCreateDir(#[from] std::io::Error),
    #[error("Can't find data dir")]
    CantFindDataDir,
    #[error("unknown error")]
    Unknown,
    #[error("Can't find config dir")]
    CantFindConfigDir,
}
