pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let len = names.len();

    let mut result: Vec<String> = Vec::with_capacity(len);

    let mut tmp: Vec<(&i32, String)> = Vec::with_capacity(len);

    for (i, w) in heights.iter().enumerate() {
        tmp.push((w, names[i].clone()));
    }

    tmp.sort_by(|a, b| b.cmp(a));

    for t in tmp {
        result.push(t.1);
    }

    result
}
