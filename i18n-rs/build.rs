fn main() {
    println!("cargo:rerun-if-env-changed=LOCALE_PATH");
    if let Some(locale_path) = option_env!("LOCALE_PATH") {
        let locale_path = i18n_find_locale::find_locale(locale_path);
        if let Some(s) = locale_path.to_str() {
            println!("cargo:rerun-if-changed={}", s);
        }
    }
}
