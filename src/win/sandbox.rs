use windows::Win32::System::SystemInformation::{
    GetSystemInfo, GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO,
};

pub fn check_cpu_with_threshold(count: u32) -> bool {
    let mut system_info: SYSTEM_INFO = unsafe { std::mem::zeroed() };

    unsafe {
        GetSystemInfo(&mut system_info);
    }

    return system_info.dwNumberOfProcessors <= count;
}

pub fn check_ram_with_threshold(gigabyte: u32) -> bool {
    let mut memory_status: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
    memory_status.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as _;

    unsafe {
        GlobalMemoryStatusEx(&mut memory_status).expect("TODO: panic message");
    }

    let min: u64 = (gigabyte as u64) * 1024 * 1024 * 1024;

    return memory_status.ullTotalPhys < min;
}
