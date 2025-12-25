use byte_unit::Byte;
use duration_str::parse as parse_duration;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, time::Duration};

#[derive(Debug, Clone, Serialize)]
pub struct TimeConfig {
    pub parsed: Duration,
    pub raw: String,
}

impl<'de> Deserialize<'de> for TimeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let raw = String::deserialize(deserializer)?;
        let parsed = parse_duration(&raw).map_err(Error::custom)?;

        Ok(TimeConfig { parsed, raw })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ByteConfig {
    pub parsed: usize,
    pub raw: String,
}

impl<'de> Deserialize<'de> for ByteConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let raw = String::deserialize(deserializer)?;
        let byte = Byte::from_str(&raw).map_err(Error::custom)?;
        let parsed = byte.as_u64() as usize;

        Ok(ByteConfig { parsed, raw })
    }
}
