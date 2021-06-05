mod load_locale;

use proc_macro::TokenStream;
use std::path::Path;

use once_cell::unsync::Lazy;

use load_locale::{load_locale, LocaleMap};

thread_local! {
    static LOCALE_MAP: Lazy<LocaleMap> = Lazy::new(load_locales_from_env);
}

fn load_locales_from_env() -> LocaleMap {
    let locale_path = option_env!("LOCALE_PATH");
    if let Some(locale_path) = locale_path.map(|path| Path::new(path)) {
        let locale_path = if locale_path.is_relative() {
            i18n_find_locale::find_locale(locale_path)
            /*find(
                Path::new(&std::env::var("OUT_DIR").map(|s| PathBuf::from(s)).unwrap_or_else(|_| std::env::current_dir().unwrap())),
                locale_path,
            )
            .unwrap()*/
        } else {
            locale_path.to_path_buf()
        };
        if !locale_path.is_dir() {
            panic!(
                "locale path is not exists or not a directory! :{:?}",
                locale_path
            );
        }
        load_locale(locale_path)
    } else {
        Default::default()
    }
}

#[proc_macro]
pub fn check_language(code: TokenStream) -> TokenStream {
    let code = syn::parse_macro_input!(code as syn::LitStr).value();
    LOCALE_MAP.with(|lm| {
        assert!(lm.contains_key(&code), "Locale {} does not exist!", &code);
    });
    TokenStream::default()
}

#[proc_macro]
pub fn check_field(field: TokenStream) -> TokenStream {
    let field = syn::parse_macro_input!(field as syn::LitStr).value();
    LOCALE_MAP.with(|lm| {
        assert!(
            lm.values().all(|lsm| lsm.contains_key(&field)),
            "Field {} does not exist!",
            &field
        );
    });
    TokenStream::default()
}

#[proc_macro]
pub fn build_match_func(_: TokenStream) -> TokenStream {
    let mut match1 = proc_macro2::TokenStream::new();

    LOCALE_MAP.with(|locale_map| {
        for (code, lsm) in locale_map.iter() {
            let mut match2 = proc_macro2::TokenStream::new();
            for (k, v) in lsm.iter() {
                match2.extend(quote::quote! {
                    #k => Some(#v),
                });
            }
            match1.extend(quote::quote! {
                #code => {
                    match field.as_ref() {
                        #match2
                        _ => None
                    }
                },
            });
        }
    });

    (quote::quote! {
        #[inline(never)]
        pub fn _match_message<S>(lang_code: S, field: S) -> Option<&'static str> where S: AsRef<str> {
            match lang_code.as_ref() {
                #match1
                _ => None
            }
        }
    })
    .into()
}
