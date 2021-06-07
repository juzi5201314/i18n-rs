// #![doc = include_str!("../readme.md")]
// wait stable 1.54.0
//! # i18n-rs
//!
//! ![Crates.io](https://img.shields.io/crates/d/simple-i18n?style=flat-square)
//! ![Lines](https://img.shields.io/tokei/lines/github/juzi5201314/i18n-rs?style=flat-square)
//! ![Crates.io](https://img.shields.io/crates/l/simple-i18n?style=flat-square)
//! [![docs.rs](https://docs.rs/simple-i18n/badge.svg)](https://docs.rs/simple-i18n)
//! [![rust-reportcard](https://rust-reportcard.xuri.me/badge/github.com/juzi5201314/i18n-rs)](https://rust-reportcard.xuri.me/report/github.com/juzi5201314/i18n-rs)
//! [![dependency status](https://deps.rs/repo/github/juzi5201314/i18n-rs/status.svg)](https://deps.rs/repo/github/juzi5201314/i18n-rs)
//!
//! A simple compile time i18n implementation in Rust.
//!
//! > *This is a personal project.
//! If you need a stable and powerful i18n library,
//! you may need [fluent](https://github.com/projectfluent/fluent-rs).*
//!
//! > If you think this crate is not easy to use, I found another similar crate: [https://github.com/terry90/internationalization-rs](https://github.com/terry90/internationalization-rs)
//!
//! ## Use
//! In crates.io, the name of this package is `simple-i18n`, because the name of `i18n-rs` is occupied by an empty crate. shit...
//!
//! Add `simple-i18n = "0.1"` to Cargo.toml
//!
//! ## Examples
//! Look [i18n-example](./examples/i18n-example)
//! ```shell
//! cd examples/i18n-example
//! LOCALE_PATH=locale cargo run --package i18n-example --bin i18n-example
//! ```
//!
//! ## [Docs](https://docs.rs/simple-i18n)
//! [docs.rs](https://docs.rs/simple-i18n)
//!
//! [Repo](https://github.com/juzi5201314/i18n-rs)
//!
//! i18n-rs will load your locale (toml, json or yaml) into the code during compilation.
//! Then you can use `lang!` (to switch the locale) and use `i18n` to get the text.
//!
//! ### LOCALE_PATH
//!
//! The `LOCALE_PATH` environment variable is used to find your locale file.
//!
//! *Please note that because the dependent library cannot get the current path where you actually run the build command during compilation, the safest method is actually to use the absolute path.*
//!
//! Usually we will store locale files in the `locale` directory in the project root directory and set `LOCALE_PATH` to `locale`.
//!
//! The current behavior is:
//!
//! >i18n-rs will find `${workspace_root}/$LOCALE_PATH` in `OUT_DIR` or `./` using `cargo metadata`.
//!
//! >In other words, if `LOCALE_PATH` is a relative path, it should be based on the workspace_root of the project, not the user's current path.
//!
//! >And it is best to run the build command in the project root directory.
//!
//! ### Locale files
//! The locale file supports `json`, `toml`, `yaml`.
//!
//! You can use a single file or use a folder. In the case of a single file, the language code is the file name.
//!
//! In the case of a folder, the folder name is language code, and the subfolder and file name will be used as the field name.
//!
//! You can add `.` in front of the file name to avoid becoming a field name.
//!
//! The content will be flattened, and the key will be linked together with `.` to become the field name.
//!
//! Example:
//! ```json
//! {
//!     "words": {
//!         "greetings": {
//!             "hi": "Hi!"
//!         }
//!     }
//! }
//! ```
//! equal
//! ```json
//! {
//!     "words.greetings.hi": "Hi!"
//! }
//! ```
//!
//! ### Strict and Loose
//! > By default, strict checking will be used.
//!
//! In loose mode, if you try to get a non-existent field or a non-existent locale, the field itself will be returned.
//!
//! But strict mode will check your input in `lang!` and `i18n!` to make sure that you are using the existing locale and fields that exist in all locales.
//!
//! If there is an error, it will be `panic!`.
//!
//! Don't worry, all of this is checked at compile time,
//! so strict checking will hardly affect runtime performance,
//! and there will be not panic at runtime.
//!
//! > note: Because it needs to be checked at compile time,
//! string literals must be used in strict mode
//!
//! Fortunately, We can freely switch between loose and strict mode.
//! like `i18n!("xxx.x"; loose)`.
//!
//! ## Benchmark
//! ```text
//! strict contrast/no strict
//!                         time:   [29.048 ns 29.387 ns 29.736 ns]
//!                         change: [-15.897% -13.053% -10.253%] (p = 0.00 < 0.05)
//!                         Performance has improved.
//! Found 1 outliers among 100 measurements (1.00%)
//!   1 (1.00%) high mild
//!
//! strict contrast/strict  time:   [29.108 ns 29.431 ns 29.776 ns]
//!                         change: [-2.6412% -0.8426% +1.0984%] (p = 0.38 > 0.05)
//!                         No change in performance detected.
//! Found 4 outliers among 100 measurements (4.00%)
//!   2 (2.00%) high mild
//!   2 (2.00%) high severe
//!
//! change_lang             time:   [148.38 ns 159.76 ns 178.01 ns]
//!                         change: [+0.4039% +4.5240% +10.326%] (p = 0.05 > 0.05)
//!                         No change in performance detected.
//! Found 5 outliers among 100 measurements (5.00%)
//!   3 (3.00%) high mild
//!   2 (2.00%) high severe
//! ```

use std::borrow::Cow;
use std::sync::Arc;

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

use i18n_macro::build_match_func;
#[doc(hidden)]
pub use i18n_macro::{check_field, check_language};

build_match_func!();

/// Get text
/// `i18n!("words.hello")`
/// `i18n!("words.hello"; loose)`
#[macro_export]
macro_rules! i18n {
    ($field:expr) => {{
        $crate::check_field!($field);
        $crate::i18n!($field; loose)
    }};
    ($field:expr; loose) => {
        $crate::_match_message($crate::Language::now().name().as_ref(), $field)
            .unwrap_or_else(|| $field)
    };
}
/// Change language
/// `lang!("en-us");`
/// `lang!("en-us"; loose);`
#[macro_export]
macro_rules! lang {
    ($lang:expr) => {
        $crate::check_language!($lang);
        $crate::lang!($lang; loose)
    };
    ($lang:expr; loose) => {
        $crate::Language::set($crate::Language::from($lang))
    };
}

static LANG: Lazy<Arc<ArcSwap<Language>>> =
    Lazy::new(|| Arc::new(ArcSwap::new(Arc::new(Language::default()))));

pub struct Language {
    name: String,
}

impl Language {
    pub fn new(code: impl ToString) -> Self {
        Language {
            name: code.to_string(),
        }
    }

    pub fn set(language: Language) {
        LANG.store(Arc::new(language))
    }

    pub fn now() -> Arc<Language> {
        Arc::clone(&LANG.load())
    }

    pub fn name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }
}

impl Default for Language {
    fn default() -> Self {
        Language {
            name: "en-us".to_owned(),
        }
    }
}

impl<T> From<T> for Language
where
    T: ToString,
{
    fn from(code: T) -> Self {
        Language::new(code)
    }
}
