use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }

    let sum = a.iter().sum::<usize>();
    let count = a.iter().filter(|i| 4 * m * *i >= sum).count();

    if count >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
