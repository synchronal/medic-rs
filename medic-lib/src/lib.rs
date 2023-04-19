pub fn std_to_string(data: Vec<u8>) -> String {
    String::from_utf8(data).unwrap()
}
