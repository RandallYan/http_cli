use std::str::FromStr;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct KvPair {
    pub key: String,
    pub value: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, '=');
        let err = || anyhow!("missing key or value in {:?}", s);
        let key = iter.next().ok_or_else(err)?;
        let value = iter.next().ok_or_else(err)?;
        Ok(Self {
            key: key.into(),
            value: value.into(),
        })
    }
}