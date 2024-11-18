#[cfg(test)]
mod test_command_to_string {
    use crate::configuration::{command::command_to_string, ARRAY_LEN};

    #[tokio::test]
    async fn test_command_to_string_normal() {
        let array: [u16; ARRAY_LEN] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = command_to_string(&array).await;
        assert_eq!(result, "1,2,3,4,5,6,7,8,9;");
    }

    #[tokio::test]
    async fn test_command_to_string_with_zeros() {
        let array: [u16; ARRAY_LEN] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = command_to_string(&array).await;
        assert_eq!(result, "0,0,0,0,0,0,0,0,0;");
    }

    #[tokio::test]
    async fn test_command_to_string_with_max_values() {
        let array: [u16; ARRAY_LEN] = [u16::MAX; ARRAY_LEN];
        let result = command_to_string(&array).await;
        assert_eq!(
            result,
            "65535,65535,65535,65535,65535,65535,65535,65535,65535;"
        );
    }
}
