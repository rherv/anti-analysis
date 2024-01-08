pub mod win;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_check_all() {
        assert_eq!(false, win::vm::check_all());
    }
}
