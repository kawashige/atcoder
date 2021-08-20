use proconio::input;

fn main() {
    input! {
        n: u64,
        h: u64,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    }

    if e * n <= h {
        println!("0");
        return;
    }

    let mut ng = 0;
    let mut ok = (e * n - h + b + e - 1) / (b + e) * a;

    while ng + 1 < ok {
        let mid = (ok + ng) / 2;

        let mut found = false;
        for i in 1..=(mid / c) {
            let j = (mid - i * c) / a;
            if i * d + j * b + h > e * (n - i - j) {
                found = true;
                break;
            }
        }

        if found {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
