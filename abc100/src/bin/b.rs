use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize
    }

    let c = if n == 100 { 101 } else { n } * 100_usize.pow(d as u32);

    println!("{}", c);
}
