pub fn split_first_off(data: &str) -> (char, &str) {
    let mut it = data.chars();
    let first = it.next().expect("vacua!");
    (first, it.as_str())
}
