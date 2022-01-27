//! Copyright (c) 2020 Nicholas Rodrigues Lordello (licensed under the Apache License, Version 2.0)
//! Modifications Copyright (c) 2022, Foris Limited (licensed under the Apache License, Version 2.0)
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use uuid::{self, Uuid};

/// The topic to identify peers or handshakes in socket messages
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Topic(String);

impl Topic {
    /// generate a new random topic
    pub fn new() -> Self {
        Topic(Uuid::new_v4().to_string())
    }

    /// generate a topic with all zeroes
    pub fn zero() -> Self {
        Topic(Uuid::nil().to_string())
    }
}

impl Default for Topic {
    fn default() -> Self {
        Topic::zero()
    }
}

impl Display for Topic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Topic {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s)?;
        Ok(Topic(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn new_topic_is_random() {
        assert_ne!(Topic::new(), Topic::new());
    }

    #[test]
    fn zero_topic() {
        assert_eq!(
            json!(Topic::zero()),
            json!("00000000-0000-0000-0000-000000000000")
        );
    }

    #[test]
    fn topic_serialization() {
        let topic = Topic::new();
        let serialized = serde_json::to_string(&topic).unwrap();
        let deserialized = serde_json::from_str(&serialized).unwrap();
        assert_eq!(topic, deserialized);
    }
}
