use proconio::input;

fn main() {
    input! {
        n: usize,
        dices: [(usize, usize); n]
    }

    if dices
        .windows(3)
        .any(|v| v[0].0 == v[0].1 && v[1].0 == v[1].1 && v[2].0 == v[2].1)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
