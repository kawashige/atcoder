use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        a: [usize; n],
        b: [usize; m]
    }

    let mut i = 0;
    let mut j = 0;
    let mut t = 0;
    let mut c = 0;

    while i < n && j < m {
        while i < n && t > a[i] {
            i += 1;
        }

        if i == n {
            break;
        }

        t = a[i] + x;

        while j < m && t > b[j] {
            j += 1;
        }

        if j == m {
            break;
        }

        t = b[j] + y;
        c += 1;
    }

    println!("{}", c);
}
