use super::ARRAY_LEN;

pub async fn command_to_string(array: &[u16; ARRAY_LEN]) -> String {
    array
        .iter()
        .map(|&value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
        + ";"
}
