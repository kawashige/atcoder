use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut bc: [(usize, usize); m]
    }

    a.sort_unstable();
    bc.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        if bc[j].1 <= a[i] {
            break;
        }
        a[i] = bc[j].1;
        bc[j].0 -= 1;
        i += 1;
        if bc[j].0 == 0 {
            j += 1;
        }
    }

    println!("{}", a.iter().sum::<usize>());
}
