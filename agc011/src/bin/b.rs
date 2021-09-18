use proconio::input;

fn check(i: usize, a: &Vec<u64>) -> bool {
    let mut x = a[i];
    for j in 0..a.len() {
        if i == j {
            continue;
        }

        if 2 * x < a[j] {
            return false;
        }
        x += a[j];
    }
    true
}

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }

    a.sort_unstable();

    if check(0, &a) {
        println!("{}", a.iter().filter(|x| x >= &&a[0]).count());
        return;
    }

    let mut ng = 0;
    let mut ok = n - 1;

    while ng + 1 < ok {
        let mid = (ok + ng) / 2;

        if check(mid, &a) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", a.iter().filter(|x| x >= &&a[ok]).count());
}
