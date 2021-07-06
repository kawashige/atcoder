use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(u64, u64); n]
    }

    let mut t = ta[0].0;
    let mut a = ta[0].1;

    for i in 1..n {
        let c = std::cmp::max((t + ta[i].0 - 1) / ta[i].0, (a + ta[i].1 - 1) / ta[i].1);

        t = c * ta[i].0;
        a = c * ta[i].1;
    }

    println!("{}", t + a);
}
