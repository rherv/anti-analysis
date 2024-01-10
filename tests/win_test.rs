use anti_analysis::win;

#[cfg(test)]
mod win_test {
    use super::win::sandbox::*;
    use super::win::vm::*;

    #[test]
    fn test_vm_check_all() {
        println!("Windows Virtual Machine Detection");
        println!("{:<13}{}", "File:", get_message(check_all_files()));
        println!("{:<13}{}", "Process:", get_message(check_all_processes()));
        println!("{:<13}{}", "RegKey:", get_message(check_all_reg_keys()));
    }

    #[test]
    fn test_sandbox_check() {
        println!("Windows Sandbox Detection");
        println!("{:<13}{}", "CPU:", get_message(check_cpu_with_threshold(2)));
        println!("{:<13}{}", "RAM:", get_message(check_ram_with_threshold(4)));
        println!("{:<13}{}", "Recent File:", get_message(check_recent_files_with_threshold(5)));
        println!("{:<13}{}", "Uptime:", get_message(check_uptime_with_threshold(5)));
    }

    fn get_message(val: bool) -> &'static str {
        if val {
            "Detected"
        } else {
            "Not Detected"
        }
    }
}
