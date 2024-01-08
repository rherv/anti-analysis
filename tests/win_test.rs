use anti_analysis::win;

#[cfg(test)]
mod win_test {
    use super::win::vm::*;

    #[test]
    fn test_vm_check_all() {
        println!("Windows Virtual Machine Check");
        println!("files\t: {}",   if check_all_files() { "detected" } else { "none" });
        println!("procs\t: {}",   if check_all_processes() { "detected" } else { "none" });
        println!("regkeys\t: {}", if check_all_reg_keys() { "detected" } else { "none" });
    }
}
