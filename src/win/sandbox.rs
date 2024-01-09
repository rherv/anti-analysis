use windows::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};

pub fn check_cpu_with_threshold(count: u32) -> bool {
    let mut system_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };

    unsafe {
        GetSystemInfo(&mut system_info);
    }

    return system_info.dwNumberOfProcessors <= count;
}