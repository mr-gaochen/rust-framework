use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serializer};
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
    // 尝试先将输入作为 Vec<String> 反序列化
    let vec_of_strings: Vec<String> = Vec::deserialize(deserializer)?;

    // 遍历每个字符串并尝试将其解析为 i64
    vec_of_strings
        .into_iter()
        .map(|s| s.parse::<i64>().map_err(de::Error::custom))
        .collect()
}

pub fn deserialize_vec_string_from_i64<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // 先将输入作为 Vec<i64> 反序列化
    let vec_of_i64: Vec<i64> = Vec::deserialize(deserializer)?;
    // 将每个 i64 转换为 String
    Ok(vec_of_i64.into_iter().map(|num| num.to_string()).collect())
}

pub fn serializer_vec_string_from_i64<S>(value: &Vec<i64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(value.len()))?;
    for &num in value {
        seq.serialize_element(&num.to_string())?;
    }
    seq.end()
}
