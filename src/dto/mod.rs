use serde::{de, Deserialize, Deserializer, Serializer};

pub mod request;
pub mod response;

/// json i64 序列化 反序列化
pub fn serialize_i64_as_str<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn serialize_option_i64_as_str<'a, S>(
    option: &'a Option<i64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match option {
        Some(value) => serializer.serialize_str(&value.to_string()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_i64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(de::Error::custom)
}

// 定义一个函数，用于将 JSON 字符串反序列化为 Option<i64>
fn deserialize_option_i64_from_str<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.is_empty() {
        // 如果字符串为空，返回 None
        Ok(None)
    } else {
        // 尝试将字符串解析为 i64
        s.parse::<i64>().map(Some).map_err(de::Error::custom)
    }
}
