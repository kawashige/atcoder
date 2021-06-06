use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut x = 1;
    for _ in 0..n {
        x += if x < k { x } else { k };
    }
    println!("{}", x);
}
