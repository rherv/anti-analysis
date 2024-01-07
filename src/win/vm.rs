pub fn check_all() -> bool {
    vbox::check_processes()
}

pub mod vbox {
    use crate::win::util::{get_processes, proc_contains};

    pub fn check_processes() -> bool {
        let vbox_processes= vec![
            "vboxservice.exe",
            "vboxtray.exe"
        ];

        !proc_contains(&get_processes(), &vbox_processes)
    }
}