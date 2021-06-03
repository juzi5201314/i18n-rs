use std::borrow::Cow;
use std::sync::Arc;

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;

use i18n_macro::build_match_func;
pub use i18n_macro::{check_field, check_language};

build_match_func!();

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
