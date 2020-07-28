use libc::{c_char, c_int, c_void, size_t, sysctl};

extern "C" {
    fn sysctlnametomib(name: *const c_char, mibp: *mut c_int, sizep: *mut size_t) -> c_int;
}

struct FanControl {
    up: [i32; 7],
    down: [i32; 7],
    level: [c_int; 4],
    level_size: size_t,
    therm: [c_int; 5],
    therm_size: size_t,
    delay: u32, //Milliseconds
    curr_level: c_int,
    cpu_temp: c_int,
}

unsafe fn control_fan(controller: &mut FanControl) {
    use std::{mem::size_of, thread, time};
    let duration = time::Duration::from_millis(controller.delay as u64);

    let mut curr_level_size = size_of::<c_int>() as size_t;
    let mut cpu_temp_size = size_of::<c_int>() as size_t;

    let mut error: c_int;

    error = sysctl(
        &controller.level[0],
        controller.level_size as u32,
        &mut controller.curr_level as *mut _ as *mut c_void,
        &mut curr_level_size,
        std::ptr::null(),
        0,
    );

    while error == 0 {
        error = sysctl(
            &controller.therm[0],
            controller.therm_size as u32,
            &mut controller.cpu_temp as *mut _ as *mut c_void,
            &mut cpu_temp_size,
            std::ptr::null(),
            0,
        );

        //println!("{}", error);
        if error != 0 {
            continue;
        }

        controller.cpu_temp = controller.cpu_temp / 10 - 273;

        while controller.curr_level > 0
            && controller.cpu_temp < controller.down[controller.curr_level as usize - 1]
        {
            controller.curr_level -= 1;
        }

        while controller.curr_level < 7
            && controller.cpu_temp > controller.up[controller.curr_level as usize]
        {
            controller.curr_level += 1;
        }

        error = sysctl(
            &controller.level[0],
            controller.level_size as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut controller.curr_level as *mut _ as *mut c_void,
            curr_level_size,
        );

        // println!("{}", error);
        if error != 0 {
            continue;
        }

        thread::sleep(duration);
    }
}

fn main() {
    use std::ffi::CStr;

    let level_name: &CStr = 
        CStr::from_bytes_with_nul(b"dev.acpi_ibm.0.fan_level\0").unwrap();
    let level_ptr = level_name.as_ptr();

    let therm_name: &CStr =
        CStr::from_bytes_with_nul(b"hw.acpi.thermal.tz0.temperature\0").unwrap();
    let therm_ptr = therm_name.as_ptr();

    let mut controller: FanControl = FanControl {
        up: [34, 38, 42, 46, 50, 54, 58],
        down: [32, 36, 40, 44, 48, 52, 56],
        delay: 100000,
        level: [0; 4],
        level_size: 4,
        therm: [0; 5],
        therm_size: 5,
        curr_level: 0,
        cpu_temp: 0,
    };

    unsafe {
        sysctlnametomib(
            level_ptr,
            &mut controller.level[0],
            &mut controller.level_size,
        );
        sysctlnametomib(
            therm_ptr,
            &mut controller.therm[0],
            &mut controller.therm_size,
        );
        //    println!("{}", controller.therm_size);
        control_fan(&mut controller);
    }
}
