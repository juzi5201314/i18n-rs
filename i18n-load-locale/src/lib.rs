use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::Context;

macro_rules! read_dir {
    ($path:expr) => {
        ::std::fs::read_dir($path)
            .with_context(|| format!("Failed to read directory: {:?}", $path))
            .unwrap()
            .map(|res| {
                res.with_context(|| format!("Failed to read directory: {:?}", $path))
                    .unwrap()
            })
    };
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

pub type LocaleStringMap = HashMap<String, String>;
pub type LocaleMap = HashMap<String, LocaleStringMap>;

pub fn load_locale<P>(path: P) -> LocaleMap
where
    P: AsRef<Path> + Debug,
{
    read_dir!(&path)
        .map(|entry| load_one_locale(entry.path()))
        .collect()
}

fn load_one_locale<P>(path: P) -> (String, LocaleStringMap)
where
    P: AsRef<Path> + Debug,
{
    let path = path.as_ref();
    if path.is_file() {
        let file_name = path
            .file_stem()
            .map(|oss| oss.to_str().map(|s| s.to_owned()))
            .flatten()
            .expect("Illegal path name");
        (
            if file_name.starts_with('.') {
                String::new()
            } else {
                file_name
            },
            load_locale_file(&path),
        )
    } else if path.is_dir() {
        let mut map = HashMap::new();
        read_dir!(&path).for_each(|entry| {
            let (name, lsm) = load_one_locale(entry.path());
            lsm.iter().for_each(|(k, v)| {
                map.insert(
                    if name.is_empty() {
                        k.to_owned()
                    } else {
                        format!("{}.{}", name, k)
                    },
                    v.to_owned(),
                );
            })
        });
        (
            path.file_name()
                .map(|oss| oss.to_str().map(|s| s.to_owned()))
                .flatten()
                .expect("Illegal path name"),
            map,
        )
    } else {
        panic!(
            "Failed to load locale: path `{:?}`: not file nor directory!",
            path
        )
    }
}

enum ConfigValue<'a> {
    Array(HashMap<String, ConfigValue<'a>>),
    String(Cow<'a, str>),
}

impl From<toml::Value> for ConfigValue<'_> {
    fn from(value: toml::Value) -> Self {
        match value {
            toml::Value::Table(table) => ConfigValue::Array(
                table
                    .into_iter()
                    .map(|(k, v)| (k, ConfigValue::from(v)))
                    .collect(),
            ),
            toml::Value::String(s) => ConfigValue::String(Cow::Owned(s)),
            other => ConfigValue::String(Cow::Owned(other.to_string())),
        }
    }
}

impl From<serde_json::Value> for ConfigValue<'_> {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Object(obj) => ConfigValue::Array(
                obj.into_iter()
                    .map(|(k, v)| (k, ConfigValue::from(v)))
                    .collect(),
            ),
            serde_json::Value::String(s) => ConfigValue::String(Cow::Owned(s)),
            other => ConfigValue::String(Cow::Owned(other.to_string())),
        }
    }
}

impl From<serde_yaml::Value> for ConfigValue<'_> {
    fn from(value: serde_yaml::Value) -> Self {
        match value {
            serde_yaml::Value::Mapping(obj) => ConfigValue::Array(
                obj.into_iter()
                    .filter_map(|(k, v)| match k {
                        serde_yaml::Value::String(s) => Some((s, ConfigValue::from(v))),
                        _ => None,
                    })
                    .collect(),
            ),
            serde_yaml::Value::String(s) => ConfigValue::String(Cow::Owned(s)),
            _ => ConfigValue::Array(Default::default()),
        }
    }
}

fn load_locale_file<P>(path_buf: P) -> HashMap<String, String>
where
    P: AsRef<Path>,
{
    let ext = path_buf
        .as_ref()
        .extension()
        .map(|oss| oss.to_str())
        .flatten()
        .unwrap_or_default();
    let file_contents = read(path_buf.as_ref())
        .with_context(|| format!("Failed to read locale file: {:?}", path_buf.as_ref()))
        .unwrap();
    match ext {
        "toml" => flatten(
            Cow::default(),
            ConfigValue::from(toml::from_slice::<toml::Value>(&file_contents).unwrap()),
        ),
        "json" => flatten(
            Cow::default(),
            ConfigValue::from(serde_json::from_slice::<serde_json::Value>(&file_contents).unwrap()),
        ),
        "yml" | "yaml" => flatten(
            Cow::default(),
            ConfigValue::from(serde_yaml::from_slice::<serde_yaml::Value>(&file_contents).unwrap()),
        ),
        _ => {
            println!("Unsupported file type: {}", ext);
            HashMap::default()
        }
    }
}

fn flatten(name: Cow<str>, val: ConfigValue) -> HashMap<String, String> {
    let mut map = HashMap::new();
    match val {
        ConfigValue::Array(array) => {
            for (name2, v) in array.into_iter() {
                map.extend(flatten(
                    Cow::Owned(if name.is_empty() {
                        name2
                    } else {
                        format!("{}.{}", name, name2)
                    }),
                    v,
                ));
            }
        }
        ConfigValue::String(s) => {
            map.insert(name.into_owned(), s.clone().into_owned());
        }
    };
    map
}

#[cfg(test)]
mod load_locale_tests {
    use crate::load_locale;

    #[test]
    fn test_load_locale() {
        let map = load_locale("examples/i18n-example/locale");
        map.get("zh-cn")
            .map(|lsm| {
                assert_eq!(lsm.get("name"), Some(&"中文".to_owned()));
                assert_eq!(lsm.get("log.level"), Some(&"日志等级".to_owned()));
                assert_eq!(lsm.get("path1.path2.path3"), Some(&"ppp".to_owned()));
                Some(())
            })
            .unwrap();
        map.get("en-us")
            .map(|lsm| {
                assert_eq!(lsm.get("name"), Some(&"English".to_owned()));
                assert_eq!(lsm.get("log.level"), Some(&"Log level".to_owned()));
                assert_eq!(lsm.get("path1.path2.path3"), Some(&"ppp".to_owned()));
                Some(())
            })
            .unwrap();
    }
}
