use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut has_gold = true;

    for i in 0..(n - 1) {
        let r = if has_gold {
            if a[i] <= a[i + 1] {
                0
            } else {
                has_gold = false;
                1
            }
        } else {
            if a[i] > a[i + 1] {
                0
            } else {
                has_gold = true;
                1
            }
        };

        println!("{}", r);
    }

    println!("{}", if has_gold { 0 } else { 1 });
}
