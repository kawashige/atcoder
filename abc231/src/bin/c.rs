use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q]
    }

    let mut r = vec![n; q];
    let mut x = x.into_iter().enumerate().collect::<Vec<_>>();

    x.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    a.sort_unstable_by(|a, b| b.cmp(&a));

    let mut j = 0;
    for i in 0..a.len() {
        while j < x.len() && x[j].1 > a[i] {
            r[x[j].0] = i;
            j += 1;
        }
    }

    for x in r {
        println!("{}", x);
    }
}
