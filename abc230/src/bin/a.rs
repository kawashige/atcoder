use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!("AGC{:0>3}", if n < 42 { n } else { n + 1 });
}
