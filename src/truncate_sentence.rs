pub fn truncate_sentence(s: String, k: i32) -> String {
    let bytes = s.as_bytes();

    let mut count = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            count += 1;
        }

        if count == k {
            return String::from(&s[..i]);
        }
    }

    return s;
}