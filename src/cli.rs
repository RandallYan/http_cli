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