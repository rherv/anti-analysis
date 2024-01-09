use anti_analysis::win;

#[cfg(test)]
mod win_test {
    use super::win::sandbox::*;
    use super::win::vm::*;

    #[test]
    fn test_vm_check_all() {
        println!("Windows Virtual Machine Check");
        println!(
            "files\t: {}",
            if check_all_files() {
                "detected"
            } else {
                "none"
            }
        );
        println!(
            "procs\t: {}",
            if check_all_processes() {
                "detected"
            } else {
                "none"
            }
        );
        println!(
            "regkeys\t: {}",
            if check_all_reg_keys() {
                "detected"
            } else {
                "none"
            }
        );
    }

    #[test]
    fn test_sandbox_check() {
        println!("Windows Sandbox Detection");
        println!(
            "cpu\t\t: {}",
            if check_cpu_with_threshold(2) {
                "detected"
            } else {
                "none"
            }
        );
        println!(
            "ram\t\t: {}",
            if check_ram_with_threshold(4) {
                "detected"
            } else {
                "none"
            }
        );
        println!(
            "recent\t: {}",
            if check_recent_files(5) {
                "detected"
            } else {
                "none"
            }
        );
    }
}
