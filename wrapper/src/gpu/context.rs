//! Shivini-side glue: apply the optional device-memory cap to a `ProverContextConfig`.

use shivini::ProverContextConfig;

use crate::gpu_config::max_device_allocation;

/// Layer the configured `max_device_allocation` (from CLI flag or env var) onto an
/// existing shivini config. A no-op when no cap is set.
pub fn apply_env_overrides(base: ProverContextConfig) -> ProverContextConfig {
    match max_device_allocation() {
        Some(bytes) => base.with_maximum_device_allocation(bytes),
        None => base,
    }
}
