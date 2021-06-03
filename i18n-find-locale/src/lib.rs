use std::path::{Path, PathBuf};

pub fn find_locale<P: AsRef<Path>>(locale_path: P) -> PathBuf {
    let root = PathBuf::from(workspace_root());
    root.join(locale_path)
}

/// https://github.com/microsoft/windows-rs/blob/master/crates/gen/src/workspace.rs#L30
fn workspace_root() -> String {
    let current_dir = std::env::current_dir().unwrap();
    if let Ok(od) = std::env::var("OUT_DIR").or_else(|_| std::env::var("CARGO_MANIFEST_DIR")) {
        std::env::set_current_dir(od).ok();
    }
    let output = std::process::Command::new(env!("CARGO"))
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps")
        .arg("--offline")
        .output()
        .expect("Failed to run `cargo metadata`");
    std::env::set_current_dir(current_dir).ok();

    const JSON_KEY: &str = r#""workspace_root":"#;
    let json = String::from_utf8(output.stdout).expect("Cargo metadata is not utf-8");
    let beginning_index = json
        .find(JSON_KEY)
        .expect("Cargo metadata did not contain `workspace_root` key.")
        + JSON_KEY.len()
        + 1;

    let ending_index = json[beginning_index..]
        .find('"')
        .expect("Cargo metadata ended before closing '\"' in `workspace_root` value");

    json[beginning_index..beginning_index + ending_index].replace("\\\\", "\\")
}
