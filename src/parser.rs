pub use self::parser::{read_config, ConfigError};

mod parser {
    use crate::level::Level;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use std::vec::Vec;

    const DEFAULT_DELAY: u32 = 2000; // two seconds

    #[derive(Debug)]
    pub enum ConfigError {
        FileError(std::io::Error),
        UnknownConfigValue(usize),
        InvalidLevelConfig(usize),
        NotEnoghLevels,
        FirstMinNotZero,
        LastMaxTooLow,
        NonascendingLevels,
        TempRangeOverlap,
        DelayInvalid,
    }

    impl From<std::io::Error> for ConfigError {
        fn from(error: std::io::Error) -> Self {
            ConfigError::FileError(error)
        }
    }

    type ConfigResult = Result<(), ConfigError>;

    pub fn read_config(file: &str, delay: &mut u32, levels: &mut Vec<Level>) -> ConfigResult {
        *delay = DEFAULT_DELAY;

        let f = File::open(file)?;
        let f = BufReader::new(f);

        for (i, l) in f.lines().enumerate() {
            let line = l.unwrap_or(String::from(""));
            let start = line.trim_start();

            if start.starts_with("level") {
                match get_level(start) {
                    Some(lvl) => {
                        levels.push(lvl);
                        Ok(())
                    }
                    None => Err(ConfigError::InvalidLevelConfig(i)),
                }?;
            } else if start.starts_with("delay") {
                match get_delay(&start[5..]) {
                    Some(n) => {
                        *delay = n;
                        Ok(())
                    }
                    None => Err(ConfigError::DelayInvalid),
                }?;
            } else if start.starts_with('#') || start.is_empty() {
            } else {
                return Err(ConfigError::UnknownConfigValue(i));
            }
        }

        sanity_check(levels)
    }

    fn sanity_check(levels: &Vec<Level>) -> ConfigResult {
        if levels.len() <= 1 {
            return Err(ConfigError::NotEnoghLevels);
        }

        if levels[0].min != 0 {
            return Err(ConfigError::FirstMinNotZero);
        }

        // 150 is an arbitrary max, I have it to support the max of the original bsdfan
        if levels[levels.len() - 1].max < 150 {
            return Err(ConfigError::LastMaxTooLow);
        }

        let mut i = 0;
        while i < levels.len() - 1 {
            if levels[i].num >= levels[i + 1].num {
                return Err(ConfigError::NonascendingLevels);
            } else if levels[i].min >= levels[i].max || levels[i].max < levels[i + 1].min {
                return Err(ConfigError::TempRangeOverlap);
            }
            i += 1;
        }

        return Ok(());
    }

    fn get_level(s: &str) -> Option<Level> {
        let start = s.find('(').unwrap_or(0); // Zeros are impossible if level is at start
        let end = s.find(')').unwrap_or(0);
        if start == 0 || end == 0 {
            return None;
        }

        let v: Vec<&str> = s[start + 1..end].split(',').collect();
        if v.len() != 3 {
            return None;
        }

        let num = v[0].trim().parse().unwrap_or(-1);
        let min = v[1].trim().parse().unwrap_or(-1);
        let max = v[2].trim().parse().unwrap_or(-1);

        if num < 0 || min < 0 || max < 0 {
            return None;
        }

        return Some(Level::new(num, min, max));
    }

    fn get_delay(s: &str) -> Option<u32> {
        match s.trim().parse() {
            Ok(n) => Some(n),
            Err(_) => None,
        }
    }
}
