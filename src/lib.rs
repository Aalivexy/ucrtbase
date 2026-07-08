//! Link Rust programs to the system `ucrtbase.dll` and eliminate the MSVC redist dependency.
//!
//! This crate automatically configures the linker to target the `ucrtbase.dll` shipped with
//! Windows 10+, removing the need for MSVC redistributable packages.
//!
//! # Supported targets
//!
//! - `x86_64-pc-windows-msvc`
//! - `i686-pc-windows-msvc`
//! - `aarch64-pc-windows-msvc`
