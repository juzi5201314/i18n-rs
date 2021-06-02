use i18n_load_locale::find;
use std::path::Path;

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
