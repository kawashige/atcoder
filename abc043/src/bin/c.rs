use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }

    let mut min = std::i32::MAX;
    for i in -100..=100 {
        let cost = a.iter().map(|x| (*x - i).pow(2)).sum::<i32>();
        min = std::cmp::min(cost, min);
    }

    println!("{}", min);
}
