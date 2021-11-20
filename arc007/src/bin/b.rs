use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        disk: [usize; m]
    }

    let mut case = (0..=n).collect::<Vec<_>>();
    case[0] = n + 1;
    let mut listen = 0;

    for d in disk {
        case[listen] = case[d];
        case[d] = n + 1;
        listen = d;
    }

    let mut r = case
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x != n + 1)
        .collect::<Vec<_>>();
    r.sort_unstable_by_key(|(_, x)| *x);

    for (x, _) in r {
        println!("{}", x);
    }
}
