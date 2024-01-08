use lazy_static::lazy_static;
use crate::win::util::fs::{get_program_files_directory, get_windows_directory};
use crate::win::util::proc::{get_running_processes, proc_contains};

lazy_static! {
    static ref WINDOWS_DIRECTORY: String = {
        get_windows_directory()
    };

    static ref PROGRAM_FILES_DIRECTORY: String = {
        get_program_files_directory()
    };
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

pub mod vbox {
    use std::path::Path;
    use crate::win::util::proc::{get_running_processes, proc_contains};
    use crate::win::util::reg::keys_exist;
    use crate::win::vm::WINDOWS_DIRECTORY;

    pub fn get_processes() -> Vec<&'static str> {
        vec![
            "vboxservice.exe",
            "vboxtray.exe"
        ]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &get_processes())
    }

    pub fn check_registry() -> bool {
        keys_exist(&vec![
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
        ].iter().any(|path_name| Path::new(&format!("{}\\{}", *WINDOWS_DIRECTORY, path_name)).exists())
    }
}

pub mod vmware {
    use std::path::Path;
    use crate::win::util::proc::{get_running_processes, proc_contains};
    use crate::win::util::reg::keys_exist;
    use crate::win::vm::WINDOWS_DIRECTORY;

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
        keys_exist(&vec![
            "SOFTWARE\\VMware, Inc.\\VMware Tools"
        ])
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
        ].iter().any(|path_name| {
            Path::new(&format!("{}\\{}", *WINDOWS_DIRECTORY, path_name)).exists()
        })
    }
}

pub mod qemu {
    use std::path::Path;
    use crate::win::util::proc::{get_running_processes, proc_contains};
    use crate::win::vm::PROGRAM_FILES_DIRECTORY;

    pub fn get_processes() -> Vec<&'static str> {
        vec![
            "qemu-ga.exe",
            "vdagent.exe",
            "vdservice.exe"
        ]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &crate::win::vm::vmware::get_processes())
    }

    pub fn check_files() -> bool {
        vec![
            "qemu-ga",
            "SPICE Guest Tools",
        ].iter().any(|path_name| Path::new(&format!("{}\\{}", *PROGRAM_FILES_DIRECTORY, path_name)).exists())
    }
}

pub mod vpc {
    use crate::win::util::proc::{get_running_processes, proc_contains};
    use crate::win::util::reg::keys_exist;

    pub fn get_processes() -> Vec<&'static str> {
        vec![
            "VMUSrvc.exe",
            "VMSrvc.exe"
        ]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &crate::win::vm::vmware::get_processes())
    }

    pub fn check_registry() -> bool {
        keys_exist(&vec![
            "SOFTWARE\\Microsoft\\Virtual Machine\\Guest\\Parameters"
        ])
    }
}

