use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        mut k: usize
    }

    for i in (1..=std::cmp::min(a, b)).rev() {
        if a % i == 0 && b % i == 0 {
            k -= 1;
            if k == 0 {
                println!("{}", i);
                return;
            }
        }
    }
}
