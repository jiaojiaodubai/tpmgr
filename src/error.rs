use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum TpmgrError {
    #[error("Package not found: {name}")]
    PackageNotFound { name: String },
    
    #[error("Version conflict: {message}")]
    VersionConflict { message: String },
    
    #[error("Dependency resolution failed: {message}")]
    DependencyResolution { message: String },
    
    #[error("Network error: {source}")]
    Network {
        #[from]
        source: reqwest::Error,
    },
    
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    
    #[error("Serialization error: {source}")]
    Serialization {
        #[from]
        source: serde_json::Error,
    },
    
    #[error("Configuration error: {source}")]
    Config {
        #[from]
        source: toml::de::Error,
    },
    
    #[error("Package integrity check failed: {name}")]
    IntegrityCheck { name: String },
    
    #[error("Permission denied: {message}")]
    Permission { message: String },
    
    #[error("Invalid package format: {message}")]
    InvalidFormat { message: String },
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, TpmgrError>;
