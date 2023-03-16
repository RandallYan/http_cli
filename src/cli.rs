use clap::Parser;
use anyhow::Result;
use reqwest::Url;

use clap::{Subcommand, Args};
use crate::kv::KvPair;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}


#[derive(Debug, Subcommand)]
pub enum Command {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Args)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    pub url: String,
}

#[derive(Debug, Args)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,
    #[arg(value_parser = parse_kv)]
    pub body: Vec<KvPair>,
}

fn parse_url(s: &str) -> Result<String>{
    let _ = s.parse::<Url>()?;
    Ok(s.into())
}

fn parse_kv(s: &str) -> Result<KvPair> {
    s.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() -> Result<(), anyhow::Error> {
        let input = "https://httpbin.org/get";
        let expected_output = "https://httpbin.org/get".to_string();
        let result = parse_url(input)?;
        assert_eq!(result, expected_output);
        Ok(())
    }

    #[test]
    fn test_parse_kv() -> Result<(), anyhow::Error> {
        let input = "key=value";
        let expected_output = KvPair {
            key: "key".to_string(),
            value: "value".to_string(),
        };
        let result = parse_kv(input)?;
        assert_eq!(result, expected_output);
        Ok(())
    }
}