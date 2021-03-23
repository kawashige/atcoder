use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        m: i32,
        a: [i32; n - 1]
    }

    let x = (n as i32) * m - a.iter().sum::<i32>();
    if x > k {
        println!("-1");
    } else {
        println!("{}", std::cmp::max(0, x));
    }
}
