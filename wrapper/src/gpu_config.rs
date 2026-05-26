//! Optional cap on shivini's GPU device-memory pool.
//!
//! Two channels feed the cap:
//! - the `ZKOS_WRAPPER_MAX_DEVICE_ALLOCATION` env var;
//! - [`set_max_device_allocation`], called from the wrapper binary's
//!   `--max-device-allocation` flag before any GPU code runs.
//!
//! The CLI setter takes precedence; the result is cached in a `OnceLock` so the value
//! is parsed once. Lives outside the `gpu` feature gate so it remains usable in
//! non-GPU builds (the CLI flag is global).

use std::sync::OnceLock;

pub const MAX_DEVICE_ALLOCATION_ENV: &str = "ZKOS_WRAPPER_MAX_DEVICE_ALLOCATION";

static MAX_DEVICE_ALLOCATION: OnceLock<Option<usize>> = OnceLock::new();

/// Cap shivini's device-memory pool at `bytes`. Idempotent and first-writer-wins, so
/// callers should invoke this before any `ProverContext::create*` call site.
pub fn set_max_device_allocation(bytes: usize) {
    let _ = MAX_DEVICE_ALLOCATION.set(Some(bytes));
}

/// Returns the cap if one was supplied via `set_max_device_allocation` or the env var.
/// Panics on env-var parse errors so misconfiguration fails fast at startup.
pub fn max_device_allocation() -> Option<usize> {
    *MAX_DEVICE_ALLOCATION.get_or_init(|| {
        let raw = std::env::var(MAX_DEVICE_ALLOCATION_ENV).ok()?;
        Some(
            parse_byte_size(&raw)
                .unwrap_or_else(|e| panic!("invalid {MAX_DEVICE_ALLOCATION_ENV}={raw:?}: {e}")),
        )
    })
}

/// Parse a Kubernetes-style byte size: `1024`, `512Mi`, `32Gi`, `32GiB`, `2G`, `2GB`, etc.
///
/// Rules:
/// - No suffix → raw bytes
/// - `K`/`M`/`G`/`T` (optionally with trailing `B`) → decimal (10^3, 10^6, 10^9, 10^12)
/// - `Ki`/`Mi`/`Gi`/`Ti` (optionally with trailing `B`) → binary (2^10, 2^20, 2^30, 2^40)
/// - Whitespace and case are ignored.
pub fn parse_byte_size(input: &str) -> Result<usize, String> {
    let s: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if s.is_empty() {
        return Err("empty value".into());
    }

    let split_at = s
        .find(|c: char| !(c.is_ascii_digit() || c == '.'))
        .unwrap_or(s.len());
    let (num, unit) = s.split_at(split_at);
    if num.is_empty() {
        return Err(format!("no numeric prefix in {input:?}"));
    }
    let value: f64 = num
        .parse()
        .map_err(|e| format!("bad number {num:?}: {e}"))?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("value must be finite and non-negative: {input:?}"));
    }

    let multiplier: u64 = match unit.to_ascii_lowercase().as_str() {
        "" | "b" => 1,
        "k" | "kb" => 1_000,
        "m" | "mb" => 1_000_000,
        "g" | "gb" => 1_000_000_000,
        "t" | "tb" => 1_000_000_000_000,
        "ki" | "kib" => 1 << 10,
        "mi" | "mib" => 1 << 20,
        "gi" | "gib" => 1 << 30,
        "ti" | "tib" => 1 << 40,
        other => return Err(format!("unknown size suffix {other:?}")),
    };

    let bytes = value * multiplier as f64;
    if bytes > usize::MAX as f64 {
        return Err(format!("value overflows usize: {input:?}"));
    }
    Ok(bytes as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_raw_bytes() {
        assert_eq!(parse_byte_size("1024").unwrap(), 1024);
        assert_eq!(parse_byte_size("0").unwrap(), 0);
    }

    #[test]
    fn parses_decimal_suffixes() {
        assert_eq!(parse_byte_size("1K").unwrap(), 1_000);
        assert_eq!(parse_byte_size("2MB").unwrap(), 2_000_000);
        assert_eq!(parse_byte_size("3 g").unwrap(), 3_000_000_000);
        assert_eq!(parse_byte_size("4tb").unwrap(), 4_000_000_000_000);
    }

    #[test]
    fn parses_binary_suffixes() {
        assert_eq!(parse_byte_size("1Ki").unwrap(), 1024);
        assert_eq!(parse_byte_size("2MiB").unwrap(), 2 << 20);
        assert_eq!(parse_byte_size("32Gi").unwrap(), 32usize << 30);
        assert_eq!(parse_byte_size("20 GiB").unwrap(), 20usize << 30);
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_byte_size("").is_err());
        assert!(parse_byte_size("GiB").is_err());
        assert!(parse_byte_size("12XB").is_err());
        assert!(parse_byte_size("-1").is_err());
    }
}
