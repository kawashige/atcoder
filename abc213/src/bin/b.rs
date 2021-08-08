use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut num = a.into_iter().enumerate().collect::<Vec<_>>();

    num.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    println!("{}", num[1].0 + 1);
}
