use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut ng = 0;
    let mut ok = n;

    while ng + 1 < ok {
        let mid = (ng + ok) / 2;

        if mid * (mid + 1) / 2 >= n {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let except = ok * (ok + 1) / 2 - n;

    for i in 1..=ok {
        if i != except {
            println!("{}", i);
        }
    }
}
