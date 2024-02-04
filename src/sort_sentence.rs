pub fn sort_sentence(s: String) -> String {

    let mut res: Vec<String> = vec![];

    let mut index1 = 0;
    let mut prev_num = 0;
    let mut prev_char = ' ';

    for (i, char) in s.bytes().enumerate() {

        if char == b' ' {
            prev_num = prev_char.to_string().parse().expect("");

            let word = String::from(&s[index1..i-1]);

            res.insert(prev_num, word);

            index1 = i+1;
        }

        prev_char = char as char;
    }

    prev_num = prev_char.to_string().parse().expect("");

    res.insert(prev_num, String::from(&s[index1..s.len() - 1]));

    let mut result = String::from("");

    for item in res.iter() {
        result.push_str(&item);
    }
    return result;
}