use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(usize, usize, usize); q]
    }

    let mut s = 0;

    for (t, x, y) in queries {
        match t {
            1 => {
                let tmp = a[(s + x - 1) % n];
                a[(s + x - 1) % n] = a[(s + y - 1) % n];
                a[(s + y - 1) % n] = tmp;
            }
            2 => s = (s + n - 1) % n,
            3 => {
                println!("{}", a[(s + x - 1) % n])
            }
            _ => unreachable!(),
        }
    }
}
