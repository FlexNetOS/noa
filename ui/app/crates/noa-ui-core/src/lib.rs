pub mod code_generator;
pub mod conversational_ai;
pub mod cross_platform;
pub mod project_manager;
pub mod ui_generator;

pub mod utils {
    pub mod compression;
    pub mod file_utils;
    pub mod hardware_detector;
    pub mod serialization;
    pub mod validation;
}

#[cfg(feature = "config-extended")]
pub mod config;

#[cfg(feature = "metadata")]
pub mod metadata;
