# anti-analysis
A rust library that provides dynamic analysis detection using a variety of techniques

## Features
### Virtual Machine Detection
- [x] Process
- [x] Registry Key
- [ ] Registry Value
- [x] File
- [x] MAC Address
- [X] Device
- [ ] Firmware
### Sandbox Detection
- [x] CPU
- [x] RAM
- [ ] HDD
- [x] Uptime Windows API
- [ ] Uptime Kernel
- [ ] Process Count
- [x] Recent Files
- [ ] Mouse
- [ ] Internet Connection
- [ ] Loaded Libraries

### Note about the difference between sandbox and vm
Virtual machine detection targets specific virtual machine artifacts, while sandbox detection focuses on general indicators that a program is operating in a virtualized environment.


## Example
```rust
use anti-analysis::win::*;

fn main() {
  /* - Virtual Machine Detection - */
  // Checks for VM file artifacts see https://github.com/rherv/anti-analysis/blob/main/src/win/vm.rs for checked files

  // IMPORTANT NOTE: Having VMware installed on a system as a host may flag this check.
  if vm::check_all_files() {
    println!("[+] detected VM file");
  }

  if vm::check_all_processes() {
    println!("[+] detected VM process");
  }

  if vm::check_all_reg_keys() {
    println!("[+] detected VM regkey");
  }

  if vm::check_all_mac_addresses() {
    println!("[+] detected VM mac address");
  }

  // IMPORTANT NOTE: Having VMware installed on a system as a host may flag this check.
  if vm::check_all_devices() {
    println!("[+] detected VM device");
  }

  /* - Sandbox Detection - */
  // Checks CPU core count, parameter is the minumum core count before beling detected.
  if sandbox::check_cpu_with_threshold(2) {
    println!("[+] detected sandbox CPU");
  }

  // Checks RAM size, parameter is the minumum GB count before beling detected.
  if sandbox::check_ram_with_threshold(4) {
    println!("[+] detected sandbox RAM");
  }

  // Checks recent files count, parameter is the minumum file count before beling detected.
  if sandbox::check_recent_files_with_threshold(20) {
    println!("[+] detected sandbox recent files");
  }

  // Checks system uptime, parameter is the minumum uptime in minutes before beling detected.
  if sandbox::check_uptime_with_threshold(5) {
    println!("[+] detected sandbox uptime");
  }
}
```

## Resources
Resources studied while making this library:
- [0xPat blog](https://0xpat.github.io/)
- [Al-Khaser](https://github.com/LordNoteworthy/al-khaser/tree/master)
