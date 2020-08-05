pub use self::controller::{Controller, FanError, FanResult};

mod controller {
    use crate::{
        level::Level,
        parser::{read_config, ConfigError},
    };
    use libc::{__error, c_int, c_void, size_t, sysctl, sysctlbyname, sysctlnametomib};
    use std::ffi::CStr;
    use std::{
        mem::size_of,
        ptr::{null, null_mut},
        vec::Vec,
    };

    const C_SIZE: size_t = size_of::<c_int>() as size_t;
    const MANUAL: i32 = 0;
    const AUTOMATIC: i32 = 1;

    #[derive(Debug)]
    pub enum FanError {
        ParsingMistake(ConfigError),
        Interupt(std::io::Error),
        SysctlError(i32),
    }

    impl From<ConfigError> for FanError {
        fn from(error: ConfigError) -> Self {
            FanError::ParsingMistake(error)
        }
    }

    impl From<std::io::Error> for FanError {
        fn from(error: std::io::Error) -> Self {
            FanError::Interupt(error)
        }
    }

    pub type FanResult = Result<(), FanError>;

    pub struct Controller {
        lvl_mib: [c_int; 4],
        lvl_size: size_t,
        tmp_mib: [c_int; 5],
        tmp_size: size_t,
        delay: u32, //Milliseconds
        curr_lvl_index: usize,
        levels: Vec<Level>,
    }

    impl Controller {
        pub fn new() -> Self {
            Controller {
                lvl_mib: [0; 4],
                lvl_size: 0,
                tmp_mib: [0; 5],
                tmp_size: 0,
                delay: 0,
                curr_lvl_index: 0,
                levels: Vec::new(),
            }
        }

        pub fn start_fan(&mut self, file: &str) -> FanResult {
            read_config(file, &mut self.delay, &mut self.levels)?;

            let fan_name: &'static CStr =
                CStr::from_bytes_with_nul(b"dev.acpi_ibm.0.fan\0").unwrap();
            let lvl_name: &'static CStr =
                CStr::from_bytes_with_nul(b"dev.acpi_ibm.0.fan_level\0").unwrap();
            let tmp_name: &'static CStr =
                CStr::from_bytes_with_nul(b"hw.acpi.thermal.tz0.temperature\0").unwrap();

            unsafe {
                if sysctlbyname(
                    fan_name.as_ptr(),
                    null_mut(),
                    null_mut(),
                    &MANUAL as *const _ as *const c_void,
                    C_SIZE,
                ) != 0
                {
                    return Err(FanError::SysctlError(*__error()));
                }

                self.lvl_size = 4;
                if sysctlnametomib(lvl_name.as_ptr(), &mut self.lvl_mib[0], &mut self.lvl_size) != 0
                {
                    return Err(FanError::SysctlError(*__error()));
                }

                self.tmp_size = 5;
                if sysctlnametomib(tmp_name.as_ptr(), &mut self.tmp_mib[0], &mut self.tmp_size) != 0
                {
                    return Err(FanError::SysctlError(*__error()));
                }

                if sysctl(
                    &self.lvl_mib[0],
                    self.lvl_size as u32,
                    null_mut(),
                    null_mut(),
                    &self.levels[0].num as *const _ as *const c_void,
                    C_SIZE,
                ) != 0
                {
                    return Err(FanError::SysctlError(*__error()));
                }

                self.curr_lvl_index = 0;
                return Ok(());
            }
        }

        pub fn drop(&mut self) {
            unsafe {
                let fan_name: &'static CStr =
                    CStr::from_bytes_with_nul(b"dev.acpi_ibm.0.fan\0").unwrap();

                sysctlbyname(
                    fan_name.as_ptr(),
                    null_mut(),
                    null_mut(),
                    &AUTOMATIC as *const _ as *const c_void,
                    C_SIZE,
                );
            }
        }

        pub fn adjust_level(&mut self, temp: c_int) -> FanResult {
            if temp < self.levels[self.curr_lvl_index].min {
                self.level_down()
            } else if temp > self.levels[self.curr_lvl_index].max {
                self.level_up()
            } else {
                Ok(())
            }
        }

        fn level_up(&mut self) -> FanResult {
            unsafe {
                self.curr_lvl_index += 1;
                if sysctl(
                    &self.lvl_mib[0],
                    self.lvl_size as u32,
                    null_mut(),
                    null_mut(),
                    &self.levels[self.curr_lvl_index].num as *const _ as *const c_void,
                    C_SIZE,
                ) != 0
                {
                    Err(FanError::SysctlError(*__error()))
                } else {
                    Ok(())
                }
            }
        }

        fn level_down(&mut self) -> FanResult {
            unsafe {
                self.curr_lvl_index -= 1;
                if sysctl(
                    &self.lvl_mib[0],
                    self.lvl_size as u32,
                    null_mut(),
                    null_mut(),
                    &self.levels[self.curr_lvl_index].num as *const _ as *const c_void,
                    C_SIZE,
                ) != 0
                {
                    Err(FanError::SysctlError(*__error()))
                } else {
                    Ok(())
                }
            }
        }

        pub fn get_temp(&self) -> Option<c_int> {
            // negative equals failure
            unsafe {
                let mut c_size_mut = C_SIZE;
                let mut temperature: c_int = 0;

                if sysctl(
                    &self.tmp_mib[0],
                    self.tmp_size as u32,
                    &mut temperature as *mut _ as *mut c_void,
                    &mut c_size_mut,
                    null(),
                    0,
                ) != 0
                {
                    return None;
                }
                return Some(temperature / 10 - 273);
            }
        }

        pub fn delay(&self) -> u32 {
            self.delay
        }
    }
}
