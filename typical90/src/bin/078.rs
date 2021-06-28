use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut count = vec![0; n];
    for (a, b) in ab {
        if a < b {
            count[b - 1] += 1;
        } else {
            count[a - 1] += 1;
        }
    }

    let r = count.iter().filter(|x| x == &&1).count();
    println!("{}", r);
}
