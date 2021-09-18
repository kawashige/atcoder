use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        k: usize,
        mut t: [usize; n]
    }

    t.sort_unstable();

    let mut bus = 1;
    let mut passenger = 1;
    let mut limit_time = t[0] + k;

    for i in 1..n {
        if c < passenger || limit_time < t[i] {
            bus += 1;
            passenger = 1;
            limit_time = t[i] + k;
        } else {
            passenger += 1;
            if c < passenger {
                bus += 1;
                passenger = 1;
                limit_time = t[i] + k;
            }
        }
    }

    println!("{}", bus);
}
