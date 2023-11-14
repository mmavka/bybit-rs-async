#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBybit<T> {
    pub ret_code: i32,
    pub ret_msg: String,
    pub result: T,
    pub ret_ext_info: RetExtInfo,
    pub time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RetExtInfo {
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub time_second: String,
    pub time_nano: String,
}

pub mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: fmt::Display,
            S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: fmt::Display,
            S: Serializer,
    {
        match value {
            Some(v) => crate::general::rest_model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(crate::general::rest_model::string_or_float::deserialize(deserializer)?))
    }
}

pub mod string_or_u64 {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: fmt::Display,
            S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrU64 {
            String(String),
            U64(u64),
        }

        match StringOrU64::deserialize(deserializer)? {
            StringOrU64::String(s) => s.parse().map_err(de::Error::custom),
            StringOrU64::U64(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: fmt::Display,
            S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}