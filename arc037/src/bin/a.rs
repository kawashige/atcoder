use proconio::input;

fn main() {
    input! {
        n: usize,
        m: [usize; n]
    }

    let r = m
        .into_iter()
        .map(|x| if x >= 80 { 0 } else { 80 - x })
        .sum::<usize>();
    println!("{}", r);
}
