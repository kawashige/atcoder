use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let sum = a.into_iter().map(|x| x - 1).sum::<usize>();
    println!("{}", sum);
}
