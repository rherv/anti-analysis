use windows::core::PCWSTR;
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{FindFirstFileW, FindNextFileW, WIN32_FIND_DATAW};
use windows::Win32::System::SystemInformation::{
    GetSystemInfo, GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO,
};
use windows::Win32::UI::Shell::{FOLDERID_Recent, SHGetKnownFolderPath, KNOWN_FOLDER_FLAG};

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

pub fn check_recent_files_with_threshold(count: u32) -> bool {
    let mut file_count: u32 = 0;

    let mut recent_files: Vec<u16>;

    unsafe {
        match SHGetKnownFolderPath(&FOLDERID_Recent, KNOWN_FOLDER_FLAG(0), None) {
            Ok(folder_path) => {
                recent_files = folder_path.as_wide().iter().copied().collect();
            }
            Err(_) => {
                return false;
            }
        }
    }

    recent_files.extend(&['\\' as u16, '*' as u16, 0]);

    let mut find_data: WIN32_FIND_DATAW = unsafe { std::mem::zeroed() };
    let file_handle: HANDLE = unsafe {
        match FindFirstFileW(PCWSTR(recent_files.as_ptr()), &mut find_data) {
            Ok(file) => file,
            Err(_) => return true,
        }
    };

    if file_handle != INVALID_HANDLE_VALUE {
        while unsafe { FindNextFileW(file_handle, &mut find_data).is_ok() } {
            file_count += 1;
        }
    }

    return file_count < count;
}
