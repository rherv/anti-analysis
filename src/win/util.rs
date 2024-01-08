use std::os::windows::ffi::OsStrExt;

pub mod proc {
    use windows::Win32::System::RemoteDesktop::{WTS_CURRENT_SERVER_HANDLE, WTS_PROCESS_INFOW, WTSEnumerateProcessesW};

    pub fn get_running_processes() -> Vec<String> {
        let mut processes: Vec<String> = Vec::new();
        let mut wts_pi: *mut WTS_PROCESS_INFOW= std::ptr::null_mut();
        let mut p_count: u32 = 0;

        unsafe {
            WTSEnumerateProcessesW(
                WTS_CURRENT_SERVER_HANDLE,
                0,
                1,
                &mut wts_pi,
                &mut p_count as *mut u32
            ).expect("TODO: panic message");

            (0..p_count).for_each(|i| {
                let process_info = &*wts_pi.offset(i as isize);
                let process_name = process_info.pProcessName.to_string().unwrap();
                processes.push(process_name);
            });
        };

        processes
    }

    pub fn proc_contains(p1: &Vec<String>, p2: &Vec<&str>) -> bool {
        p1.iter().any(|proc1| p2.iter().any(|proc2| proc1 == proc2))
    }
}

pub mod reg {
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::System::Registry::{HKEY_LOCAL_MACHINE, RegOpenKeyW};
    use windows::core::*;
    use windows::Win32::Foundation::ERROR_SUCCESS;
    use crate::win::util::encode_wide;

    pub fn keys_exist(keys: &Vec<&str>) -> bool {
        keys.iter().any(|key| key_exists(key))
    }

    fn key_exists(key: &str) -> bool {
        let mut hkey = unsafe { std::mem::zeroed() };

        let err = unsafe { RegOpenKeyW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(encode_wide(key).as_ptr()),
            &mut hkey,
        )};

        match err {
            Ok(_) => {
                true
            }
            Err(err) => {
                if err == Error::from(ERROR_SUCCESS) {
                    return true;
                }
                false
            }
        }
    }
}

pub mod fs {
    use windows::Win32::System::SystemInformation::GetWindowsDirectoryW;

    pub fn get_windows_directory() -> String {
        let mut output_size: u32 = 0;
        let mut windows_directory: Vec<u16> = std::iter::repeat('\0' as u16)
            .take(1024)
            .collect();

        unsafe {
            output_size = GetWindowsDirectoryW(
                Some(windows_directory.as_mut_slice())
            );
        }

        windows_directory.truncate(output_size as usize);

        String::from_utf16_lossy(&*windows_directory)
    }
}


fn encode_wide(s: &str) -> Vec<u16> {
    std::ffi::OsString::from(s)
        .encode_wide()
        .chain(Some(0))
        .collect()
}