use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut a: [usize; n]
    }

    let mut c = vec![vec![0; w]; h];
    let mut l = 0;

    for i in 0..h {
        for j in 0..w {
            let k = if i % 2 == 0 { j } else { w - 1 - j };

            c[i][k] = l + 1;
            a[l] -= 1;
            if a[l] == 0 {
                l += 1;
            }
        }
    }

    for i in 0..h {
        println!(
            "{}",
            c[i].iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
