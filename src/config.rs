use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;
use void::Void;

pub const DEFAULT_CONF: &'static str = "goodcheck.yml";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rule {
    pub id: String,
    #[serde(deserialize_with = "string_or_struct")]
    pub pattern: RulePattern,
    pub message: String,
    pub justification: Option<Vec<String>>,
    pub glob: Option<Vec<String>>,
    pub pass: Option<Vec<String>>,
    pub fail: Option<Vec<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RulePattern {
    pub string: Option<String>,
    pub regexp: Option<String>,
    pub literal: Option<String>,
    pub token: Option<String>,
    #[serde(default = "RulePattern::default_case_sensitive")]
    pub case_sensitive: bool,
    #[serde(default = "RulePattern::default_multiline")]
    pub multiline: bool,
}

impl RulePattern {
    fn default_case_sensitive() -> bool {
        true
    }

    fn default_multiline() -> bool {
        false
    }
}

impl FromStr for RulePattern {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RulePattern {
            string: Some(s.to_string()),
            ..RulePattern::default()
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, visitor: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
    let file = File::open(path)?;
    let conf = serde_yaml::from_reader(file)?;
    Ok(conf)
}
