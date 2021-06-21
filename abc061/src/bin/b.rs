use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }

    let mut count = vec![0; n];
    for (a, b) in ab {
        count[a - 1] += 1;
        count[b - 1] += 1;
    }

    for x in count {
        println!("{}", x);
    }
}
