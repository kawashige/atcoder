use proconio::input;

fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128,
        mut k: i128
    }

    let mut sum = 0;
    if k <= a {
        println!("{}", k);
        return;
    } else {
        sum += a;
        k -= a;
        if k <= b {
            println!("{}", sum);
            return;
        }
        k -= b;
        sum -= k;
        println!("{}", sum)
    }
}
