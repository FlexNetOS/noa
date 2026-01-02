//! Settings feature module.
//!
//! Provides settings management for the NOA platform.

mod settings_page;
mod provider_settings;
mod appearance_settings;
mod about;

pub use settings_page::SettingsPage;
pub use provider_settings::ProviderSettings;
pub use appearance_settings::AppearanceSettings;
pub use about::AboutPage;
