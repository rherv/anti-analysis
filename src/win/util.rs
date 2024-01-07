use windows::Win32::System::RemoteDesktop::{WTS_CURRENT_SERVER_HANDLE, WTS_PROCESS_INFOW, WTSEnumerateProcessesW};

pub fn get_processes() -> Vec<String> {
    let mut processes: Vec<String> = Vec::new();
    let mut wts_pi: *mut WTS_PROCESS_INFOW= std::ptr::null_mut();
    let mut p_count: u32 = 0;

    unsafe {
        WTSEnumerateProcessesW(
            WTS_CURRENT_SERVER_HANDLE,
            0,
            1,
            &mut wts_pi,
            //wts_pi.as_mut() as *mut _ as _,
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