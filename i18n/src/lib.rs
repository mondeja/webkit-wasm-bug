//! Localization system of simple-icons website

use leptos::RwSignal;
use std::collections::HashMap;

::lazy_static::lazy_static! {
    static ref TRANSLATIONS: HashMap<&'static str, HashMap<String, String>> = {
        let mut translations = HashMap::new();
        let mut es = HashMap::new();
        es.insert("{} free {} icons for popular brands".to_string(), "{} iconos {} gratis para marcas populares".to_string());
        translations.insert("es".into(), es);
        translations
    };
}

#[derive(Clone, Copy)]
pub struct Language {
    /// Language code
    pub code: &'static str,
    /// Language name
    pub name: &'static str,
}

pub static LANGUAGES: [Language; 2] = [
    Language {
        code: "en",
        name: "English",
    },
    Language {
        code: "es",
        name: "Español",
    },
];

impl Language {
    pub fn translate(&self, key: &'static str) -> String {
        TRANSLATIONS
            .get(self.code)
            .and_then(|translations| translations.get(key))
            .unwrap_or(&key.to_string())
            .to_string()
    }

    pub fn from_str(code: &str) -> Option<Self> {
        match LANGUAGES.iter().find(|lang| lang.code == code) {
            Some(language) => Some(*language),
            None => None,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        LANGUAGES[0]
    }
}

#[derive(Copy, Clone)]
pub struct LocaleStateSignal(pub RwSignal<Language>);

#[macro_export]
macro_rules! gettext_impl {
    ($cx:ident, $key:expr) => {
        (&use_context::<::i18n::LocaleStateSignal>($cx)
            .expect("The locale context has not been provided")
            .0
            .get()
            .translate($key))
            .to_string()
    };
}

#[macro_export]
macro_rules! replace_impl {
    ($key:expr, $($replacements:expr),+) => {
        {
            let mut string = $key.to_string();
            $(
                string = string.replacen("{}", $replacements, 1);
            )+
            string
        }
    };
}

#[macro_export]
macro_rules! gettext {
    ($cx:ident, $key:expr) => {
        $crate::gettext_impl!($cx, $key)
    };
    ($cx:ident, $key:expr, $($replacements:expr),+) => {
        $crate::replace_impl!($crate::gettext_impl!($cx, $key), $($replacements),+)
    };
}

#[macro_export]
macro_rules! move_gettext {
    ($cx:ident, $key:expr) => {
        move||$crate::gettext!($cx, $key)
    };
    ($cx:ident, $key:expr, $($replacements:expr),+) => {
        move||$crate::gettext!($cx, $key, $($replacements),+)
    };
}
