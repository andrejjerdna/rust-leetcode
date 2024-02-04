pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {

    let index = k as usize;
    score.sort_by(|a, b| b[index].cmp(&a[index]));
    score
}