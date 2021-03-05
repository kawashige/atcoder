use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [usize; n]
    }

    p.sort_unstable();
    println!("{}", p.iter().take(k).sum::<usize>());
}
