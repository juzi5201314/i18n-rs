//! A simple compile time i18n implementation in Rust.
//!
//! [Repo](https://github.com/juzi5201314/i18n-rs)
//!
//! i18n-rs will load your locale (toml, json or yaml) into the code during compilation.
//! Then you can use `lang!` (to switch the locale) and use `i18n` to get the text.
//!
//! *[Example](https://github.com/juzi5201314/i18n-rs/blob/master/examples/i18n-example)*
//!
//! ## LOCALE_PATH
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
//! ## Locale files
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
//! ## Strict and Loose
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
            .unwrap_or_else(|| ::std::borrow::Cow::Borrowed($field))
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
