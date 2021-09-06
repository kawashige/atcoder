use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut red = vec![false; n];
    let mut count = vec![1; n];
    red[0] = true;

    for (x, y) in xy {
        if red[x - 1] {
            red[y - 1] = true;
        }

        count[x - 1] -= 1;
        count[y - 1] += 1;

        if count[x - 1] == 0 {
            red[x - 1] = false;
        }
    }

    println!("{}", (0..n).filter(|i| red[*i]).count());
}
