use std::path::PathBuf;
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

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let def_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("def");

    let lib = find_msvc_tools::find_tool(arch, "lib.exe")
        .map(|t| t.path().to_path_buf())
        .expect("lib.exe not found; ensure Visual Studio or MSVC build tools are installed");

    for (name, def) in [
        ("vcruntime", def_dir.join(format!("vcruntime_{arch}.def"))),
        ("ucrt", def_dir.join(format!("ucrt_{arch}.def"))),
    ]
    .into_iter()
    .chain((arch != "x86").then(|| ("vcruntime.weak", def_dir.join("vcruntime.weak.def"))))
    {
        let output = out_dir.join(format!("{name}.lib"));
        let status = Command::new(&lib)
            .args([
                &format!("/def:{}", def.display()),
                &format!("/out:{}", output.display()),
                &format!("/machine:{arch}"),
            ])
            .status()?;
        if !status.success() {
            return Err(format!("lib.exe exited with {status}").into());
        }
        println!("cargo:rerun-if-changed={}", def.display());
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    Ok(())
}
