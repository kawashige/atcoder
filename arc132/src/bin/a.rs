use proconio::input;

fn main() {
    input! {
        n: usize,
        r: [usize; n],
        c: [usize; n],
        q: usize,
        rc: [(usize, usize); q]
    }

    println!(
        "{}",
        rc.into_iter()
            .map(|(qr, qc)| if n - r[qr - 1] + 1 <= c[qc - 1] {
                '#'
            } else {
                '.'
            })
            .collect::<String>()
    );
}
