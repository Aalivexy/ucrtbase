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

    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    let lib = find_msvc_tools::find_tool(arch, "lib.exe")
        .map(|t| t.path().to_path_buf())
        .expect("lib.exe not found");
    let def = root.join("def").join(format!("vcruntime_{arch}.def"));
    let output = out_dir.join(format!("vcruntime.lib"));
    let status = Command::new(&lib)
        .arg(format!("/def:{}", def.display()))
        .arg(format!("/out:{}", output.display()))
        .arg(format!("/machine:{arch}"))
        .status()?;
    if !status.success() {
        return Err(format!("lib.exe exited with {status} while generating vcruntime.lib").into());
    }
    println!("cargo:rerun-if-changed={}", def.display());

    if arch != "x86" {
        let asm = root.join("asm").join("weak.asm");
        let obj = out_dir.join("weak.obj");

        let masm_exe = match arch {
            "x64" => "ml64.exe",
            _ => "armasm64.exe",
        };
        let masm = find_msvc_tools::find_tool(arch, masm_exe)
            .map(|t| t.path().to_path_buf())
            .expect("masm not found");

        let status = Command::new(&masm)
            .arg("/c")
            .arg(format!("/Fo{}", obj.display()))
            .arg(&asm)
            .status()?;
        if !status.success() {
            return Err(format!("masm exited with {status} while compiling weak.asm").into());
        }
        let status = Command::new(&lib)
            .arg(&out_dir.join("vcruntime.lib"))
            .arg(&obj)
            .status()?;
        if !status.success() {
            return Err(format!("lib.exe exited with {status} while merging weak.obj").into());
        }
        println!("cargo:rerun-if-changed={}", asm.display());
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    Ok(())
}
