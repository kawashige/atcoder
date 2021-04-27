use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();

    let mut min = a[0];

    for i in 1..n {
        let mut m = a[i] % min;
        if m == 0 {
            continue;
        }
        while min % m != 0 {
            let tmp = min;
            min = m;
            m = tmp % m;
        }
        min = m;
    }

    println!("{}", min)
}
