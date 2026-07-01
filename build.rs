use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") {
        return Ok(());
    }

    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH")?.as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "aarch64" => "arm64",
        _ => return Ok(()),
    };

    let out = PathBuf::from(std::env::var("OUT_DIR")?);
    let base = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let defs = base.join("def");
    let stubs = base.join("stubs").join("stubs.c");

    let stub_lib_name = format!("ucrt_stubs_{arch}");
    cc::Build::new().file(&stubs).compile(&stub_lib_name);
    let stub_lib = out.join(format!("{stub_lib_name}.lib"));

    let lib = find_lib()?;

    let vcrt = out.join("vcruntime.lib");
    run(
        &lib,
        &[
            &format!(
                "/def:{}",
                defs.join(format!("vcruntime_{arch}.def")).display()
            ),
            &format!("/out:{}", vcrt.display()),
            &format!("/machine:{arch}"),
        ],
    )?;

    run(
        &lib,
        &[
            vcrt.to_string_lossy().as_ref(),
            stub_lib.to_string_lossy().as_ref(),
            &format!("/out:{}", vcrt.display()),
        ],
    )?;

    let ucrt = out.join("ucrt.lib");
    run(
        &lib,
        &[
            &format!("/def:{}", defs.join(format!("ucrt_{arch}.def")).display()),
            &format!("/out:{}", ucrt.display()),
            &format!("/machine:{arch}"),
        ],
    )?;

    println!("cargo:rustc-link-search={}", out.display());

    for f in [
        &stubs,
        &defs.join(format!("vcruntime_{arch}.def")),
        &defs.join(format!("ucrt_{arch}.def")),
    ] {
        println!("cargo:rerun-if-changed={}", f.display());
    }

    Ok(())
}

fn find_lib() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tool = cc::Build::new().get_compiler();
    if let Some(parent) = tool.path().parent()
        && let lib = parent.join("lib.exe")
        && lib.exists()
    {
        return Ok(lib);
    }
    Err("lib.exe not found".into())
}

fn run<I, S>(prog: &Path, args: I) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(prog).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{} exited with {status}", prog.display()).into())
    }
}
