#[cfg(test)]
mod test_command_to_string {
    use tokio::runtime::Builder;

    use crate::configuration::command::command_to_string;

    #[test]
    fn to_string() {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let array: [u16; 9] = [11, 0, 10, 0, 100, 0, 4, 0, 0];
        let result = runtime.block_on(command_to_string(&array));

        assert_eq!(result, "11,0,10,0,100,0,4,0,0;")
    }
}
