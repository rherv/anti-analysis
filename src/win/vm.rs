use crate::win::util::proc::{get_running_processes, proc_contains};

pub fn check_all() -> bool {
    check_all_processes() || check_all_reg_keys()
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
    vbox::check_registry()
}

pub mod vbox {
    use crate::win::util::proc::{get_running_processes, proc_contains};
    use crate::win::util::reg::keys_exist;

    pub fn get_reg_keys() -> Vec<&'static str> {
        vec![
            "HARDWARE\\ACPI\\DSDT\\VBOX__",
            "HARDWARE\\ACPI\\FADT\\VBOX__",
            "HARDWARE\\ACPI\\RSDT\\VBOX__",
            "SOFTWARE\\Oracle\\VirtualBox Guest Additions",
            "SYSTEM\\ControlSet001\\Services\\VBoxGuest",
            "SYSTEM\\ControlSet001\\Services\\VBoxMouse",
            "SYSTEM\\ControlSet001\\Services\\VBoxService",
            "SYSTEM\\ControlSet001\\Services\\VBoxSF",
            "SYSTEM\\ControlSet001\\Services\\VBoxVideo",
        ]
    }

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
        println!("check_registry called");
        keys_exist(&get_reg_keys())
    }
}

pub mod vmware {
    use crate::win::util::proc::{get_running_processes, proc_contains};

    pub fn get_reg_keys() -> Vec<&'static str> {
        vec![
            "SOFTWARE\\VMware, Inc.\\VMware Tools"
        ]
    }

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
}

pub mod qemu {
    use crate::win::util::proc::{get_running_processes, proc_contains};

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
}

pub mod vpc {
    use crate::win::util::proc::{get_running_processes, proc_contains};

    pub fn get_reg_keys() -> Vec<&'static str> {
        vec![
            "SOFTWARE\\Microsoft\\Virtual Machine\\Guest\\Parameters"
        ]
    }

    pub fn get_processes() -> Vec<&'static str> {
        vec![
            "VMUSrvc.exe",
            "VMSrvc.exe"
        ]
    }

    pub fn check_processes() -> bool {
        proc_contains(&get_running_processes(), &crate::win::vm::vmware::get_processes())
    }
}

