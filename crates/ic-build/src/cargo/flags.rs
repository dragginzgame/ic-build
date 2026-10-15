//! Cargo flag composition; callers retain flag selection and build policy.

#[cfg(test)]
mod tests;

use std::env;

/// Append arguments to Cargo's effective inherited Rust flags.
///
/// `CARGO_ENCODED_RUSTFLAGS` takes precedence over `RUSTFLAGS`, including when
/// explicitly empty. Ordinary flags are split on whitespace, matching Cargo;
/// additions retain their argument boundaries through unit-separator encoding.
/// Non-Unicode environment values are treated as unavailable.
///
/// Returns `None` without reading either variable when `extra_flags` is empty:
/// leave the command's inherited flags unchanged in that case. Otherwise set
/// the returned string as `CARGO_ENCODED_RUSTFLAGS` on the Cargo command. The
/// caller chooses additions, target-specific flags and the command environment.
#[must_use]
pub fn combined_encoded_rustflags(extra_flags: &[String]) -> Option<String> {
    if extra_flags.is_empty() {
        return None;
    }

    let mut combined = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_else(|_| {
        env::var("RUSTFLAGS")
            .unwrap_or_default()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("\x1f")
    });
    for flag in extra_flags {
        if !combined.is_empty() {
            combined.push('\x1f');
        }
        combined.push_str(flag);
    }

    Some(combined)
}
