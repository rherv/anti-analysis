use lazy_static::lazy_static;
use std::ffi::OsString;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use windows::core::{Error, PCWSTR};
use windows::core::imp::{GetProcessHeap, HeapAlloc};
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Networking::WinSock::AF_UNSPEC;
use windows::Win32::NetworkManagement::IpHelper::{GET_ADAPTERS_ADDRESSES_FLAGS, IP_ADAPTER_ADDRESSES_LH};
use windows::Win32::System::Registry::{RegOpenKeyW, HKEY_LOCAL_MACHINE};
use windows::Win32::System::RemoteDesktop::{
    WTSEnumerateProcessesW, WTS_CURRENT_SERVER_HANDLE, WTS_PROCESS_INFOW,
};
use windows::Win32::System::SystemInformation::GetWindowsDirectoryW;
use windows::Win32::UI::Shell::CSIDL_PROGRAM_FILES;

lazy_static! {
    static ref WINDOWS_DIRECTORY: String = get_windows_directory();
    static ref PROGRAM_FILES_DIRECTORY: String = get_program_files_directory();
}

pub fn check_all() -> bool {
    check_all_processes() || check_all_reg_keys() || check_all_files()
}

pub fn check_all_processes() -> bool {
    let mut vm_processes: Vec<&str> = Vec::new();
    vm_processes.extend(vbox::get_processes());
    vm_processes.extend(vmware::get_processes());
    vm_processes.extend(qemu::get_processes());
    vm_processes.extend(vpc::get_processes());

    proc_contains(&get_running_processes(), &vm_processes)
}

pub fn check_all_reg_keys() -> bool {
    vbox::check_registry() || vmware::check_registry() || vpc::check_registry()
}

pub fn check_all_files() -> bool {
    vbox::check_files() || vmware::check_files() || qemu::check_files()
}

pub fn check_all_mac_addresses() -> bool {
    vbox::check_mac_addresses()
}

pub mod vbox {
    use crate::win::vm::*;
    use std::path::Path;

    pub fn get_processes() -> Vec<&'static str> {
        vec!["vboxservice.exe", "vboxtray.exe"]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &get_processes())
    }

    pub fn check_registry() -> bool {
        any_keys_exist(&vec![
            "HARDWARE\\ACPI\\DSDT\\VBOX__",
            "HARDWARE\\ACPI\\FADT\\VBOX__",
            "HARDWARE\\ACPI\\RSDT\\VBOX__",
            "SOFTWARE\\Oracle\\VirtualBox Guest Additions",
            "SYSTEM\\ControlSet001\\Services\\VBoxGuest",
            "SYSTEM\\ControlSet001\\Services\\VBoxMouse",
            "SYSTEM\\ControlSet001\\Services\\VBoxService",
            "SYSTEM\\ControlSet001\\Services\\VBoxSF",
            "SYSTEM\\ControlSet001\\Services\\VBoxVideo",
        ])
    }

    pub fn check_files() -> bool {
        vec![
            "System32\\drivers\\VBoxMouse.sys",
            "System32\\drivers\\VBoxGuest.sys",
            "System32\\drivers\\VBoxSF.sys",
            "System32\\drivers\\VBoxVideo.sys",
            "System32\\vboxdisp.dll",
            "System32\\vboxhook.dll",
            "System32\\vboxmrxnp.dll",
            "System32\\vboxogl.dll",
            "System32\\vboxoglarrayspu.dll",
            "System32\\vboxoglcrutil.dll",
            "System32\\vboxoglerrorspu.dll",
            "System32\\vboxoglfeedbackspu.dll",
            "System32\\vboxoglpackspu.dll",
            "System32\\vboxoglpassthroughspu.dll",
            "System32\\vboxservice.exe",
            "System32\\vboxtray.exe",
            "System32\\VBoxControl.exe",
        ]
        .iter()
        .any(|path_name| Path::new(&format!("{}\\{}", *WINDOWS_DIRECTORY, path_name)).exists())
    }

    pub fn check_mac_addresses() -> bool {
        valid_mac_addresses(
            vec![
                [0x08, 0x00, 0x27], // Virtual Box MAC Address
            ]
        )
    }
}

pub mod vmware {
    use crate::win::vm::*;
    use std::path::Path;

    pub fn get_processes() -> Vec<&'static str> {
        vec![
            "vmtoolsd.exe",
            "vmwaretray.exe",
            "vmwareuser.exe",
            "VGAuthService.exe",
            "vmacthlp.exe",
        ]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &get_processes())
    }

    pub fn check_registry() -> bool {
        any_keys_exist(&vec!["SOFTWARE\\VMware, Inc.\\VMware Tools"])
    }

    pub fn check_files() -> bool {
        vec![
            "System32\\drivers\\vmnet.sys",
            "System32\\drivers\\vmmouse.sys",
            "System32\\drivers\\vmusb.sys",
            "System32\\drivers\\vm3dmp.sys",
            "System32\\drivers\\vmci.sys",
            "System32\\drivers\\vmhgfs.sys",
            "System32\\drivers\\vmmemctl.sys",
            "System32\\drivers\\vmx86.sys",
            "System32\\drivers\\vmrawdsk.sys",
            "System32\\drivers\\vmusbmouse.sys",
            "System32\\drivers\\vmkdb.sys",
            "System32\\drivers\\vmnetuserif.sys",
            "System32\\drivers\\vmnetadapter.sys",
        ]
        .iter()
        .any(|path_name| Path::new(&format!("{}\\{}", *WINDOWS_DIRECTORY, path_name)).exists())
    }
}

pub mod qemu {
    use crate::win::vm::*;
    use std::path::Path;

    pub fn get_processes() -> Vec<&'static str> {
        vec!["qemu-ga.exe", "vdagent.exe", "vdservice.exe"]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &get_processes())
    }

    pub fn check_files() -> bool {
        vec!["qemu-ga", "SPICE Guest Tools"]
            .iter()
            .any(|path_name| {
                Path::new(&format!("{}\\{}", *PROGRAM_FILES_DIRECTORY, path_name)).exists()
            })
    }
}

pub mod vpc {
    use crate::win::vm::*;

    pub fn get_processes() -> Vec<&'static str> {
        vec!["VMUSrvc.exe", "VMSrvc.exe"]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &get_processes())
    }

    pub fn check_registry() -> bool {
        any_keys_exist(&vec![
            "SOFTWARE\\Microsoft\\Virtual Machine\\Guest\\Parameters",
        ])
    }
}

fn get_running_processes() -> Vec<String> {
    let mut processes: Vec<String> = Vec::new();
    let mut wts_pi: *mut WTS_PROCESS_INFOW = std::ptr::null_mut();
    let mut p_count: u32 = 0;

    unsafe {
        match WTSEnumerateProcessesW(
            WTS_CURRENT_SERVER_HANDLE,
            0,
            1,
            &mut wts_pi,
            &mut p_count as *mut u32,
        ) {
            Ok(_) => {}
            Err(_) => {
                return Vec::new();
            }
        }

        (0..p_count).for_each(|i| {
            let process_info = &*wts_pi.offset(i as isize);
            let process_name = OsString::from_wide(process_info.pProcessName.as_wide())
                .to_string_lossy()
                .as_ref()
                .to_string();
            processes.push(process_name);
        });
    };

    processes
}

fn proc_contains(p1: &Vec<String>, p2: &Vec<&str>) -> bool {
    p1.iter().any(|proc1| p2.iter().any(|proc2| proc1 == proc2))
}

fn any_keys_exist(keys: &Vec<&str>) -> bool {
    keys.iter().any(|key| key_exists(key))
}

fn key_exists(key: &str) -> bool {
    let mut hkey = unsafe { std::mem::zeroed() };

    let err = unsafe {
        RegOpenKeyW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(encode_wide(key).as_ptr()),
            &mut hkey,
        )
    };

    match err {
        Ok(_) => true,
        Err(err) => {
            if err == Error::from(ERROR_SUCCESS) {
                return true;
            }
            false
        }
    }
}

fn get_windows_directory() -> String {
    let output_size: u32;
    let mut windows_directory: Vec<u16> = std::iter::repeat('\0' as u16).take(1024).collect();

    unsafe {
        output_size = GetWindowsDirectoryW(Some(windows_directory.as_mut_slice()));
    }

    windows_directory.truncate(output_size as usize);

    String::from_utf16_lossy(&*windows_directory)
}

fn get_program_files_directory() -> String {
    use windows::Win32::UI::Shell::SHGetFolderPathW;
    let mut path: [u16; 260] = [0; 260];

    unsafe {
        let _ = SHGetFolderPathW(None, CSIDL_PROGRAM_FILES as i32, None, 0, &mut path);
    }

    OsString::from_wide(&path)
        .to_string_lossy()
        .as_ref()
        .trim_end_matches('\0')
        .to_string()
}

pub fn valid_mac_addresses(mac_adresses: Vec<[u8; 3]>) -> bool {
    use windows::Win32::NetworkManagement::IpHelper::GetAdaptersAddresses;
    use windows::Win32::System::Memory::HEAP_ZERO_MEMORY;
    let mut adapter_list_size: u32 = 0;

    unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC.0 as u32,
            GET_ADAPTERS_ADDRESSES_FLAGS(0),
            None,
            None,
            &mut adapter_list_size,
        );
    }

    let ip_adapter_addresses = unsafe {
        HeapAlloc(
            GetProcessHeap(),
            HEAP_ZERO_MEMORY.0,
            adapter_list_size as usize,
        ) as *mut IP_ADAPTER_ADDRESSES_LH
    };

    unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC.0 as u32,
            GET_ADAPTERS_ADDRESSES_FLAGS(0),
            None,
            Some(ip_adapter_addresses),
            &mut adapter_list_size,
        );
    }
    let mac_address = unsafe {
        // unsafe dereference
        match ip_adapter_addresses.as_ref() {
            None => return false,
            Some(&mac) => {
                [
                    mac.PhysicalAddress[0],
                    mac.PhysicalAddress[1],
                    mac.PhysicalAddress[2],
                ]
            },
        }
    };

    mac_adresses.iter().any(|&mac| mac_address == mac)
}

fn encode_wide(s: &str) -> Vec<u16> {
    std::ffi::OsString::from(s)
        .encode_wide()
        .chain(Some(0))
        .collect()
}
