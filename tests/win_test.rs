use anti_analysis::win;

#[cfg(test)]
mod win_test {
    use super::win::sandbox::*;
    use super::win::vm::*;
    use crate::msg;

    #[test]
    fn test_vm_check_all() {
        println!("Windows Virtual Machine Detection");
        println!("{:<20}{}", "File:", msg(check_all_files()));
        println!("{:<20}{}", "Process:", msg(check_all_processes()));
        println!("{:<20}{}", "RegKey:", msg(check_all_reg_keys()));
        println!("{:<20}{}", "Mac Address:", msg(check_all_mac_addresses()));
        println!("{:<20}{}", "Devices:", msg(check_all_devices()));
    }

    #[test]
    fn test_sandbox_check() {
        println!("Windows Sandbox Detection");
        println!("{:<20}{}", "CPU:", msg(check_cpu_with_threshold(2)));
        println!("{:<20}{}", "RAM:", msg(check_ram_with_threshold(4)));
        println!(
            "{:<20}{}",
            "Recent File:",
            msg(check_recent_files_with_threshold(5))
        );
        println!("{:<20}{}", "Uptime:", msg(check_uptime_with_threshold(5)));
    }
}

fn msg(val: bool) -> &'static str {
    if val {
        "Detected"
    } else {
        "Not Detected"
    }
}
