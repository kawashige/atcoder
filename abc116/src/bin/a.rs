use proconio::input;

fn main() {
    input! {
        ab: usize,
        bc: usize,
        _ca: usize,
    }

    println!("{}", (ab * bc) / 2);
}
