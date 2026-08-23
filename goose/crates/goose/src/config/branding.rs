//! Single source of truth for this fork's product name.
//!
//! Changing `APP_SLUG` reroutes the on-disk config/data/state directories and
//! the system keyring service, so this build keeps its state fully separate
//! from an upstream `goose` install. `APP_DISPLAY_NAME` is used for
//! user-facing text (CLI help, banners).

/// Filesystem/shell-safe identifier: binary name, config/data directory name,
/// and keyring service name.
pub const APP_SLUG: &str = "cloudwolf";

/// Human-facing product name shown in help output and messages.
pub const APP_DISPLAY_NAME: &str = "CloudWolf";
