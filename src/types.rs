use chrono::{DateTime, Utc};
pub use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};

pub type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Solve {
    pub time: f64,
    pub comment: String,
    pub scramble: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub plus_two: bool,
    pub dnf: bool,
}

#[derive(Serialize, Debug, Clone)]
pub struct Ao5Solve {
    pub last_solve: Solve,
    pub time: f64,
    pub none: bool,
    pub dnf: bool,
}

#[derive(Parser, Debug)]
pub struct Cli {
    pub sort_order: SortOrder,
    pub sort_by: SortBy,
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Clone, Debug)]
pub enum SortBy {
    Single,
    Ao5,
}

#[derive(Clone, Debug)]
pub enum SortOrder {
    Latest,
    Oldest,
    Best,
    Worst,
}

impl FromStr for SortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "single" => Ok(SortBy::Single),
            "ao5" => Ok(SortBy::Ao5),
            _ => Err("not a valid sort by".to_string()),
        }
    }
}

impl FromStr for SortOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "latest" => Ok(SortOrder::Latest),
            "oldest" => Ok(SortOrder::Oldest),
            "best" => Ok(SortOrder::Best),
            "worst" => Ok(SortOrder::Worst),
            _ => Err("not a valid sort order".to_string()),
        }
    }
}
