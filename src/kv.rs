use std::str::FromStr;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvpair_from_str() -> anyhow::Result<(), anyhow::Error> {
        let input = "key=value";
        let expected_output = KvPair {
            key: "key".to_string(),
            value: "value".to_string(),
        };
        let result = KvPair::from_str(input)?;
        assert_eq!(result, expected_output);
    
        let input2 = "key_without_value=";
        let expected_output2 = KvPair {
            key: "key_without_value".to_string(),
            value: "".to_string(),
        };
        let result2 = KvPair::from_str(input2)?;
        assert_eq!(result2, expected_output2);

        Ok(())
    }
    
}