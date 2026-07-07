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

    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    let lib = find_msvc_tools::find_tool(arch, "lib.exe")
        .map(|t| t.path().to_path_buf())
        .expect("lib.exe not found");
    let def = root.join("def").join(format!("vcruntime_{arch}.def"));
    let output = out_dir.join("vcruntime.lib");
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
            .arg(out_dir.join("vcruntime.lib"))
            .arg(&obj)
            .status()?;
        if !status.success() {
            return Err(format!("lib.exe exited with {status} while merging weak.obj").into());
        }
        println!("cargo:rerun-if-changed={}", asm.display());
    }

    if arch == "x86" {
        fix_x86_name_type(&output)?;
    }

    println!("cargo:rustc-link-search={}", out_dir.display());
    Ok(())
}

fn fix_x86_name_type(lib_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    use object::LittleEndian as LE;
    use object::pe::{self, ImportObjectHeader};
    use object::read::archive::ArchiveFile;
    use std::mem::offset_of;

    let data = std::fs::read(lib_path)?;

    let patch_offsets: Vec<usize> = {
        let archive = ArchiveFile::parse(&*data)?;
        let mut offsets = Vec::new();

        for member in archive.members() {
            let member = member?;
            let member_data = member.data(&*data)?;

            let mut off = 0u64;
            let Ok(header) = ImportObjectHeader::parse(member_data, &mut off) else {
                continue;
            };

            let raw = header.name_type.get(LE);
            let name_type = (raw >> pe::IMPORT_OBJECT_NAME_SHIFT) & pe::IMPORT_OBJECT_NAME_MASK;
            if name_type != pe::IMPORT_OBJECT_NAME_NO_PREFIX {
                continue;
            }

            let name = &member_data[off as usize..];
            let end = name.iter().position(|&b| b == 0).unwrap_or(name.len());
            if !name[..end].contains(&b'@') {
                continue;
            }

            let member_start = member_data.as_ptr() as usize - data.as_ptr() as usize;
            offsets.push(member_start + offset_of!(ImportObjectHeader, name_type));
        }

        offsets
    };

    if !patch_offsets.is_empty() {
        let mut out = data;
        for &pos in &patch_offsets {
            let raw = u16::from_le_bytes([out[pos], out[pos + 1]]);
            let name_type_field = pe::IMPORT_OBJECT_NAME_UNDECORATE << pe::IMPORT_OBJECT_NAME_SHIFT;
            let name_type_mask = pe::IMPORT_OBJECT_NAME_MASK << pe::IMPORT_OBJECT_NAME_SHIFT;
            let fixed = (raw & !name_type_mask) | name_type_field;
            out[pos..pos + 2].copy_from_slice(&fixed.to_le_bytes());
        }
        std::fs::write(lib_path, &out)?;
    }

    Ok(())
}
