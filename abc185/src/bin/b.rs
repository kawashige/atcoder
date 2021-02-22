use proconio::input;

fn main() {
    input! {
        n: i32,
        m: usize,
        t: i32,
        cafe: [(i32, i32); m],
    }

    let mut r = n;
    for i in 0..m {
        r -= cafe[i].0 - if i == 0 { 0 } else { cafe[i - 1].1 };
        if r <= 0 {
            println!("No");
            return;
        }
        r = std::cmp::min(n, r + cafe[i].1 - cafe[i].0);
    }
    r -= t - cafe[m - 1].1;
    if r <= 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
