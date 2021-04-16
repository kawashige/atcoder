use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [[usize]; m],
        p: [usize; m]
    }

    let mut count = 0;

    for i in 0..(1 << n) {
        if (0..m).all(|j| {
            (0..s[j].len())
                .filter(|k| i & 1 << (s[j][*k] - 1) != 0)
                .count()
                % 2
                == p[j]
        }) {
            count += 1;
        }
    }

    println!("{}", count);
}
