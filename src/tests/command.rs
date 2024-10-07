#[cfg(test)]
mod test_command_to_string {
    use tokio::runtime::Builder;

    use crate::configuration::command::command_to_string;

    #[test]
    fn positive_numbers() {
        let runtime = Builder::new_current_thread().enable_all().build().unwrap();

        let array: [u16; 9] = [11, 0, 10, 0, 100, 0, 4, 0, 0];
        let result = runtime.block_on(command_to_string(&array));

        assert_eq!(result, "11,0,10,0,100,0,4,0,0;")
    }
}

// #[cfg(test)]
// mod test_get_keypad_port {
//     use tokio::runtime::Builder;

//     use crate::configuration::{command_to_string, ARRAY_LEN};

//     #[test]
//     fn keypad_port() {
//         let runtime = Builder::new_current_thread().enable_all().build().unwrap();

//         runtime.block_on({
//             let write_data_array: [u16; ARRAY_LEN] = [101, 0, 0, 0, 0, 0, 0, 0, 0];
//             let write_data = command_to_string(&write_data_array).await;
//             write_data
//         })
//     }
// }
