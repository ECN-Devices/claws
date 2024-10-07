use super::ARRAY_LEN;

pub async fn command_to_string(array: &[u16; 9]) -> String {
    let mut command_string = String::new();

    for (i, &value) in array.iter().take(ARRAY_LEN).enumerate() {
        command_string.push_str(&value.to_string());

        if i + 1 != ARRAY_LEN {
            command_string.push(',')
        } else {
            command_string.push(';')
        }
    }

    command_string
}
