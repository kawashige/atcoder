use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut count = 0;
    for i in a..=b {
        if i.to_string() == i.to_string().chars().rev().collect::<String>() {
            count += 1;
        }
    }
    println!("{}", count);
}
