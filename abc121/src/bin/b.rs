use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n]
    }

    let count = (0..n)
        .filter(|i| (0..m).map(|j| a[*i][j] * b[j]).sum::<i32>() + c > 0)
        .count();

    println!("{}", count);
}
