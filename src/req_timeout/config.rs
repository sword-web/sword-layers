use duration_str::parse as parse_duration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::DisplayConfig;

/// ### Request Timeout Configuration
/// Configuration for the Request Timeout Layer
/// This configuration allows you to set a maximum duration for request handling.
///
/// #### Fields:
/// - `enabled`: A boolean indicating if request timeout is enabled.
/// - `duration`: A string representing the timeout duration (e.g., "30s", "5m").
/// - `parsed`: The parsed duration in std::time::Duration derived from `duration`.
/// - `display`: A boolean indicating if the configuration should be displayed.
#[derive(Debug, Clone, Serialize)]
pub struct RequestTimeoutConfig {
    pub enabled: bool,
    pub duration: String,

    #[serde(skip)]
    pub parsed: Duration,

    #[serde(default)]
    pub display: bool,
}

impl DisplayConfig for RequestTimeoutConfig {
    fn display(&self) {
        if !self.display {
            return;
        }

        if self.enabled {
            println!("  ↳  Request Timeout: {}", self.duration);
        } else {
            println!("  ↳  Request Timeout: disabled");
        }
    }
}

impl<'de> Deserialize<'de> for RequestTimeoutConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};
        use std::fmt;

        struct TimeoutLimitVisitor;

        impl<'de> Visitor<'de> for TimeoutLimitVisitor {
            type Value = RequestTimeoutConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a map with 'enabled' (bool), 'duration' (string), and optional 'display' (bool) fields",
                )
            }

            // Deserialize from a map/object
            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut enabled = None;
                let mut duration = None;
                let mut display = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "enabled" => enabled = Some(map.next_value()?),
                        "duration" => duration = Some(map.next_value()?),
                        "display" => display = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let enabled = enabled.ok_or_else(|| Error::missing_field("enabled"))?;
                let duration: String = duration.ok_or_else(|| Error::missing_field("duration"))?;
                let parsed = parse_duration(&duration).map_err(Error::custom)?;

                Ok(RequestTimeoutConfig {
                    enabled,
                    duration,
                    parsed,
                    display: display.unwrap_or_else(|| false),
                })
            }
        }

        deserializer.deserialize_any(TimeoutLimitVisitor)
    }
}
