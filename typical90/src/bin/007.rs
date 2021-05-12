use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        b: [usize; q]
    }

    a.sort_unstable();

    for x in b {
        let r = match a.binary_search(&x) {
            Ok(_) => 0,
            Err(i) => {
                if i == 0 {
                    a[0] - x
                } else if i == n {
                    x - a[n - 1]
                } else {
                    std::cmp::min(x - a[i - 1], a[i] - x)
                }
            }
        };
        println!("{}", r);
    }
}
