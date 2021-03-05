use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let all = a.iter().fold(0, |acc, i| acc ^ i);
    for i in a {
        println!("{}", i ^ all);
    }
}
