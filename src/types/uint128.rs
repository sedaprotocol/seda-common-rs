#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uint128(pub u128);

impl Uint128 {
    pub fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }
}

impl core::fmt::Display for Uint128 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u128> for Uint128 {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl serde::Serialize for Uint128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for Uint128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let u = s.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(u))
    }
}
