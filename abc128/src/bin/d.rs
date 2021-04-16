use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        v: [i32; n]
    }

    let vv = v.iter().chain(v.iter()).cloned().collect::<Vec<i32>>();

    let mut max = 0;

    for i in 0..std::cmp::min(k, n) {
        for j in i..std::cmp::min(k, n) {
            let mut v = vv[(n + i - j)..=(n + i)].to_vec();
            v.sort_unstable();
            let c = k - j - 1;
            let tmp_max = v
                .into_iter()
                .zip(1..)
                .skip_while(|(x, i)| i <= &c && x < &0)
                .map(|(x, _)| x)
                .sum::<i32>();

            max = std::cmp::max(max, tmp_max);
        }
    }

    println!("{}", max);
}
