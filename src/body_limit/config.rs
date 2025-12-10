use byte_unit::Byte;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::DisplayConfig;

/// ### Body Limit Configuration
/// Configuration for the Body Limit Layer
/// This configuration allows you to set a maximum size for request bodies.
///
/// #### Fields:
/// - `max_size`: A string representing the maximum allowed size for request bodies (e.g
/// "1MB", "500KB").
///
/// - `parsed`: The parsed size in bytes (usize) derived from `max_size`.
/// - `display`: A boolean indicating if the configuration should be displayed.
#[derive(Debug, Clone, Serialize)]
pub struct BodyLimitConfig {
    pub max_size: String,

    #[serde(skip)]
    pub parsed: usize,

    #[serde(default)]
    pub display: bool,
}

impl DisplayConfig for BodyLimitConfig {
    fn display(&self) {
        if self.display {
            println!("  ↳  Max Body Size: {}", self.max_size);
        }
    }
}

impl Default for BodyLimitConfig {
    fn default() -> Self {
        let max_size = "10MB".to_string();
        let parsed = Byte::from_str(&max_size)
            .unwrap_or_else(|_| Byte::from_u64(10 * 1024 * 1024))
            .as_u64();

        BodyLimitConfig {
            display: true,
            max_size,
            parsed: parsed as usize,
        }
    }
}

impl<'de> Deserialize<'de> for BodyLimitConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};
        use std::fmt;

        struct BodyLimitVisitor;

        impl<'de> Visitor<'de> for BodyLimitVisitor {
            type Value = BodyLimitConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a map with 'enabled' (bool), 'max_size' (string), and optional 'display' (bool) fields",
                )
            }

            // Deserialize from a map/object
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut max_size = None;
                let mut display = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "max_size" => max_size = Some(map.next_value()?),
                        "display" => display = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let max_size: String = max_size.ok_or_else(|| Error::missing_field("max_size"))?;

                let parsed = Byte::from_str(&max_size)
                    .map(|b| b.as_u64() as usize)
                    .map_err(Error::custom)?;

                Ok(BodyLimitConfig {
                    max_size,
                    parsed,
                    display: display.unwrap_or_else(|| true),
                })
            }
        }

        deserializer.deserialize_any(BodyLimitVisitor)
    }
}
