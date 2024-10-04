use std::os::raw::{c_int, c_uint};
use std::thread;
use std::time::Duration;

#[link(name = "System")]
extern "C" {
    fn mach_host_self() -> u32;
    fn host_statistics(
        host_priv: u32,
        flavor: c_int,
        host_info_out: *mut host_cpu_load_info,
        host_info_outCnt: *mut u32,
    ) -> c_int;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct host_cpu_load_info {
    cpu_ticks: [u32; 4],
}

const CP_USER: usize = 0;
const CP_SYS: usize = 1;
const CP_NICE: usize = 2;
const CP_IDLE: usize = 3;

const HOST_CPU_LOAD_INFO: c_int = 3;
const HOST_CPU_LOAD_INFO_COUNT: u32 = 4;
const KERN_SUCCESS: c_int = 0;

fn get_cpu_count() -> u32 {
    unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) as u32 }
}

fn _get_cpu_percentage() -> Option<host_cpu_load_info> {
    let mut count = HOST_CPU_LOAD_INFO_COUNT;
    let mut info = host_cpu_load_info { cpu_ticks: [0; 4] };

    let result = unsafe {
        host_statistics(
            mach_host_self(),
            HOST_CPU_LOAD_INFO,
            &mut info as *mut host_cpu_load_info,
            &mut count as *mut u32,
        )
    };

    if result != KERN_SUCCESS {
        None
    } else {
        Some(info)
    }
}

pub fn cpu_percentage(cpu_usage_delay: u64) -> Option<f32> {
    let load1 = _get_cpu_percentage()?;
    thread::sleep(Duration::from_micros(cpu_usage_delay));
    let load2 = _get_cpu_percentage()?;

    let mut total = 0u64;
    let mut active = 0u64;

    for i in 0..4 {
        let diff = load2.cpu_ticks[i].wrapping_sub(load1.cpu_ticks[i]) as u64;
        total += diff;
        if i != CP_IDLE {
            active += diff;
        }
    }

    if total == 0 {
        return Some(0.0);
    }

    Some((active as f64 / total as f64 * 100.0) as f32)
}

fn main() {
    println!("Number of CPUs: {}", get_cpu_count());

    // Print CPU usage every second for 5 seconds
    for _ in 0..5 {
        match cpu_percentage(100_000_000) {
            Some(usage) => println!("CPU usage: {:.1}%", usage),
            None => println!("Failed to get CPU usage"),
        }
        thread::sleep(Duration::from_secs(1));
    }
}
