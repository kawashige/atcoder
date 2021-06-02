use proconio::input;

fn main() {
    input! {
        n: usize,
        f: [[usize; 10]; n],
        p: [[i32; 11]; n]
    }

    let f = f
        .into_iter()
        .map(|v| (0..10).fold(0, |acc, i| if v[i] == 1 { acc | 1 << i } else { acc }))
        .collect::<Vec<usize>>();

    let mut max = std::i32::MIN;
    for i in 1..(1 << 10) {
        let mut profit = 0;
        for j in 0..n {
            let c = (i & f[j]).count_ones() as usize;
            profit += p[j][c];
        }
        max = std::cmp::max(max, profit);
    }

    println!("{}", max);
}
