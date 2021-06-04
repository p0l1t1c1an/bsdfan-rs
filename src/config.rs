use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;
use thiserror::Error;

const DEFAULT_DELAY: u64 = 2000; // two seconds

#[derive(Debug, Deserialize)]
pub struct Level {
    #[serde(alias = "number", alias = "lvl", alias = "level")]
    num: i32,

    #[serde(alias = "minimum", alias = "min_temp")]
    min: i32,
    
    #[serde(alias = "maximum", alias = "max_temp")]
    max: i32,
}

impl Level {
    pub fn num(&self) -> i32 {
        self.num
    }
    pub fn min(&self) -> i32 {
        self.min
    }
    pub fn max(&self) -> i32 {
        self.max
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    FileError(#[from] std::io::Error),

    #[error("{0}")]
    TomlError(#[from] toml::de::Error),
    
    #[error("Your config file needs at least one level set")]
    NotEnoghLevels,
    
    #[error("Multiple level {0}'s were set to the at indexes {1} and {2} post sort")]
    RepeatedLevels(i32, usize, usize),

    #[error("Levels were improperly sorted. Level {0} came before {1} at indexes {2} and {3} post sort")]
    NonAscendingLevels(i32, i32, usize, usize),
 
    #[error("Level {0}'s min temp {1}C is larger than it's max temp {2}C")]
    MinLargerThanMax(i32, i32, i32),   

    #[error("Level {0}'s max temp {1}C doesn't go past level {2}'s min temp {3}C")]
    RangesDoNotOverlap(i32, i32, i32, i32),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(alias = "delay_millis")]
    delay: Option<u64>,

    #[serde(alias = "Levels")]
    levels: Vec<Level>,
}

type ConfigResult<T> = Result<T, ConfigError>;

impl Config {
    pub fn delay(&self) -> u64 {
        self.delay.unwrap_or(DEFAULT_DELAY)
    }

    pub fn levels(&self) -> &Vec<Level> {
        &self.levels
    }

    pub fn new(file: &str) -> ConfigResult<Self> {
        let mut config_string = String::new();
        File::open(file).and_then(|mut f| f.read_to_string(&mut config_string))?;
        let mut config : Self = toml::from_str(&config_string)?;
        config.levels.sort_unstable_by(|a,b| a.num().cmp(&b.num()));
        Self::sanity_check(config.levels())?;
        Ok(config)
    }

    fn sanity_check(levels: &Vec<Level>) -> ConfigResult<()> {
        if levels.len() <= 1 {
            return Err(ConfigError::NotEnoghLevels);
        }

        let mut i = 0;
        while i < levels.len() - 1 {
            let curr_num = levels[i].num();
            let curr_min = levels[i].min();
            let curr_max = levels[i].max();
            
            let next_num = levels[i].num();
            let next_min = levels[i].min();

            if curr_num == next_num {
                return Err(ConfigError::RepeatedLevels(curr_num, i, i+1));
            } else if curr_num > next_num {
                return Err(ConfigError::NonAscendingLevels(curr_num, next_num, i, i+1));
            } else if curr_min >= curr_max {
                return Err(ConfigError::MinLargerThanMax(curr_num, curr_min, curr_max));
            } else if curr_max < next_min {
                return Err(ConfigError::RangesDoNotOverlap(curr_num, curr_max, next_num, next_min));
            }

            i += 1;
        }

        Ok(())
    }
}

