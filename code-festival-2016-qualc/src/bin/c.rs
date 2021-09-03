use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n]
    }

    let mut r: u64 = 1;

    if (1..n).all(|i| a[i] == a[i - 1]) && (1..n).all(|i| b[i] == b[i - 1]) && a[0] != b[0] {
        println!("0");
        return;
    }

    for i in 1..(n - 1) {
        if (a[i - 1] < a[i] && a[i] > b[i]) || (b[i] > b[i + 1] && a[i] < b[i]) {
            println!("0");
            return;
        } else if a[i - 1] == a[i] && b[i] == b[i + 1] {
            r *= a[i].min(b[i])
        };
        r %= 1_000_000_007;
    }

    println!("{}", r);
}
