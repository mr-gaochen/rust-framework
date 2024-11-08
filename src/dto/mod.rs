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

pub fn serialize_option_i64_as_str<'a, S>(
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

pub fn deserialize_i64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(de::Error::custom)
}

// 定义一个函数，用于将 JSON 字符串反序列化为 Option<i64>
pub fn deserialize_option_i64_from_str<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str: Option<String> = Option::deserialize(deserializer)?;

    match opt_str {
        Some(s) if !s.is_empty() => s.parse::<i64>().map(Some).map_err(de::Error::custom),
        _ => Ok(None), // 字符串为空或字段缺失时返回 None
    }
}

pub fn deserialize_vec_i64_from_str<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    if s.is_empty() {
        // 如果字符串为空，返回空的 Vec
        Ok(Vec::new())
    } else {
        // 拆分字符串并尝试将每个部分解析为 i64
        s.split(',')
            .map(|part| part.trim().parse::<i64>().map_err(de::Error::custom))
            .collect()
    }
}

pub fn deserialize_vec_string_from_i64<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    if s.is_empty() {
        Ok(Vec::new())
    } else {
        Ok(s.split(',').map(|part| part.trim().to_string()).collect())
    }
}
