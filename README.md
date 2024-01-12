# anti-analysis library
A rust library that provides dynamic analysis detection using a variety of techniques

## Example
```rust
use anti-analysis::win::*;

fn main() {
  /* - Virtual Machine Detection - */
  // Checks for VM file artifacts see https://github.com/rherv/anti-analysis/blob/main/src/win/vm.rs for checked files
  // NOTE: Having VMware installed on a system may flag this check
  if vm::check_all_files() {
    println!("[+] detected VM file");
  }

  // Checks for VM specific processes
  if vm::check_all_processes() {
    println!("[+] detected VM process");
  }

  // Checks for VM specific registry keys
  if vm::check_all_reg_keys() {
    println!("[+] detected VM regkey");
  }

  /* - Sandbox Detection - */
  // Checks CPU core count, parameter is the minumum core count before beling detected
  if sandbox::check_cpu_with_threshold(2) {
    println!("[+] detected sandbox CPU");
  }

  // Checks RAM size, parameter is the minumum GB count before beling detected
  if sandbox::check_ram_with_threshold(4) {
    println!("[+] detected sandbox RAM");
  }

  // Checks recent files count, parameter is the minumum file count before beling detected
  if sandbox::check_recent_files_with_threshold(20) {
    println!("[+] detected sandbox recent files");
  }

  // Checks system uptime, parameter is the minumum uptime in minutes before beling detected
  if sandbox::check_uptime_with_threshold(5) {
    println!("[+] detected sandbox uptime");
  }
}
```

## Resources
Resources studied while making this library:
- [0xPat blog](https://0xpat.github.io/)
- [Al-Khaser](https://github.com/LordNoteworthy/al-khaser/tree/master)
