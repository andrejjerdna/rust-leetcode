mod sort_the_students;

fn main() {
    let input = vec![vec![10,6,9,1],vec![7,5,11,2],vec![4,8,3,15]];

    let res = sort_the_students::sort_the_students(input, 2);

    println!("{:?}", res);
}
