use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
        b: [i32; n]
    }

    let n = (0..n).map(|i| (a[i] - b[i]).abs() as usize).sum::<usize>();
    if n <= k && (n == k || (k - n) % 2 == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
