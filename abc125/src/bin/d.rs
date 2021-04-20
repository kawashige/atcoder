use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let sum = a.iter().map(|i| i.abs()).sum::<i64>();
    let min = a.iter().map(|i| i.abs()).min().unwrap();
    let minus_count = a.iter().filter(|i| i < &&0).count();

    if minus_count == 0 || minus_count % 2 == 0 {
        println!("{}", sum);
    } else {
        println!("{}", sum - (min * 2));
    }
}
