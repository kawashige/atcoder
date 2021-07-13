use proconio::input;

fn main() {
    input! {
        a: usize,
        mut b: usize,
        mut c: usize,
        k: usize,
    }

    let mut count = 0;
    while a >= b {
        b *= 2;
        count += 1;
    }

    while b >= c {
        c *= 2;
        count += 1;
    }

    if count <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
