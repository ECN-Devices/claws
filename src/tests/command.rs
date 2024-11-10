#[cfg(test)]
mod test_command_to_string {
    use crate::configuration::command::command_to_string;

    #[tokio::test]
    async fn to_string() {
        let array: [u16; 9] = [11, 0, 10, 0, 100, 0, 4, 0, 0];
        let result = command_to_string(&array).await;

        assert_eq!(result, "11,0,10,0,100,0,4,0,0;")
    }
}
