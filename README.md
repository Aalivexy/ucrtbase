# ucrtbase

Link Rust programs to the system `ucrtbase.dll` and eliminate the MSVC redist dependency.

## Usage

```sh
cargo add ucrtbase
```

Or add to `Cargo.toml`:

```toml
ucrtbase = "0.1"
```

Once added, the crate automatically configures the linker to target the system `ucrtbase.dll`. No source code changes are required.

## Motivation

This crate is inspired by [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5), but targets Windows 10 and later.

VC-LTL5 maintains backward compatibility with older Windows by reimplementing many functions already present in `ucrtbase.dll` on modern systems. While you can raise VC-LTL5's minimum supported version to work around this, two issues remain:

- The [`vc-ltl` crate](https://crates.io/crates/vc-ltl) cannot distribute all static libraries due to crates.io size limits. For x86/x64, it only ships versions compatible with Windows `6.0.6000+`, which links against `msvcrt.dll` instead of `ucrtbase.dll` — many functions still get bundled into your binary even though they exist in the system library.
- Using upstream VC-LTL5 directly works, but requires downloading the [90MB+ nupkg](https://www.nuget.org/api/v2/package/VC-LTL5/) and extracting only a small portion of its contents — plus you'd need to write your own `build.rs` or configure `RUSTFLAGS` to link against it.

This crate is ~10KB, contains no precompiled static libraries, and achieves the same result with minimal overhead.

## Compatibility

For pure Rust projects this is not a concern — compatibility issues only arise from linked C/C++ code compiled with MSVC.

Compatibility depends on whether you statically link MSVC-compiled code, your compiler version, and compiler flags. In short, if your code doesn't reference any symbols absent from the `ucrtbase.dll` shipped with Windows `10.0.10240` (e.g., `__C_specific_handler_noexcept`, `__CxxFrameHandler4`, `__uncaught_exceptions`), it is compatible down to `10.0.10240`. Otherwise, `10.0.19041+` is required. Notably, `__CxxFrameHandler4` is emitted by default in C++ code compiled with VS2019+, though it can be disabled via compiler flag.

### Supported targets

- `x86_64-pc-windows-msvc`
- `i686-pc-windows-msvc`
- `aarch64-pc-windows-msvc`

## Limitations

The following are not exported by `ucrtbase.dll` and therefore cannot be provided by this crate. If you need them, consider using a statically linked CRT (e.g., `+crt-static`):

- `c11_atomic_support`
- `c11_threads`

## Special Thanks

- [VC-LTL5](https://github.com/Chuyu-Team/VC-LTL5) — the project that inspired this crate.
