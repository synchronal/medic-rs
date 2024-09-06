pub fn to_str(command: &String, dir: &Option<String>) -> String {
  match dir {
    Some(dir) => format!("(cd {} && {})", dir, command),
    None => format!("({})", command),
  }
}
