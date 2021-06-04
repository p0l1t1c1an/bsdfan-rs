use crate::config::{Config, ConfigError};
use sysctl::{Ctl, CtlValue, Sysctl, SysctlError};

const MANUAL: CtlValue = CtlValue::Int(0);
const AUTOMATIC: CtlValue = CtlValue::Int(1);

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FanError {
    #[error("{0}")]
    ConfigError(#[from] ConfigError),
    
    #[error("{0}")]
    SysctlError(#[from] SysctlError),

    // Errors retroactively done for Signal handler in main
    #[error("{0}")]
    SignalError(#[from] std::io::Error),

    #[error("Temp Ctl didn't return an i32 value")]
    WrongTempType,
}

pub type FanResult<T> = Result<T, FanError>;

pub struct Controller {
    fan_ctl: Ctl,
    level_ctl: Ctl,
    temp_ctl: Ctl,
    curr_index: usize,
    config: Config,
}

impl Controller {
    pub fn new(file: &str) -> FanResult<Self> {
        Ok(Controller {
            fan_ctl: Ctl::new("dev.acpi_ibm.0.fan")?,
            level_ctl: Ctl::new("dev.acpi_ibm.0.fan_level")?,
            temp_ctl: Ctl::new("hw.acpi.thermal.tz0.temperature")?,
            curr_index: 0,
            config : Config::new(file)?,
        })
    }

    pub fn start(&mut self) -> FanResult<()> {
        self.fan_ctl.set_value(MANUAL)?;
        let curr_level = &self.config.levels()[self.curr_index];
        self.level_ctl.set_value(CtlValue::Int(curr_level.num()))?;
        Ok(())
    }

    pub fn stop(&mut self) -> FanResult<()> {
        self.fan_ctl.set_value(AUTOMATIC)?;
        Ok(())
    }

    pub fn adjust_level(&mut self, temp: i32) -> FanResult<()> {
        let levels = self.config.levels();
        let curr_level = &levels[self.curr_index];

        if self.curr_index > 0 && temp < curr_level.min() {
            self.curr_index -= 1;
        } else if self.curr_index < levels.len() -1 && temp > curr_level.max() {
            self.curr_index += 1;
        } else {
            return Ok(());
        }

        let new_level = &levels[self.curr_index];
        self.level_ctl.set_value(CtlValue::Int(new_level.num()))?;
        Ok(())
    }

    pub fn get_temp(&self) -> FanResult<i32> {
        if let CtlValue::Int(temp) = self.temp_ctl.value()? {
            Ok((temp  / 10) - 273)
        } else {
            Err(FanError::WrongTempType)
        }
    }

    pub fn delay(&self) -> u64 {
        self.config.delay()
    }
}

