use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("{}", (1..(n as u64)).sum::<u64>());
}
