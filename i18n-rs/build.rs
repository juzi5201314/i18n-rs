use std::io;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=LOCALE_PATH");
    if let Some(locale_path) = option_env!("LOCALE_PATH") {
        let locale_path = find(
            Path::new(&std::env::var("OUT_DIR").unwrap()),
            locale_path.as_ref(),
        )
        .unwrap();
        if let Some(s) = locale_path.to_str() {
            println!("cargo:rerun-if-changed={}", s);
        }
    }
}

// Recursively find the target path in the parent directory.
pub fn find<P>(directory: P, target_path: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let p = directory.as_ref().join(&target_path);

    match std::fs::metadata(&p) {
        Ok(metadata) => {
            if metadata.is_dir() {
                return Ok(p);
            }
        }
        Err(error) => {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error);
            }
        }
    }

    if let Some(parent) = directory.as_ref().parent() {
        find(parent, target_path.as_ref())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("path: `{:?}` not found", target_path.as_ref()),
        ))
    }
}
