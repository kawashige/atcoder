use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    a.sort_unstable();

    if a.len() == 2 {
        println!("{} {}", a[1], a[0])
    } else {
        let i = match a[..(n - 1)].binary_search(&(a[n - 1] / 2)) {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    0
                } else if i == n - 2 {
                    n - 2
                } else if a[n - 1] / 2 - a[i - 1] < a[i] - a[n - 1] / 2 {
                    i - 1
                } else {
                    i
                }
            }
        };
        println!("{} {}", a[n - 1], a[i]);
    }
}
