use proconio::input;

fn main() {
    input! {
        n: u64
    }

    if n == 1 {
        println!("Aoki");
        return;
    }

    let mut i: u64 = 2;
    let mut x = 1;
    let mut c = 1;

    while x < n {
        x += i;
        i *= 2;
        c += 1;
    }

    if c % 2 == 0 {
        let mut x = 1;
        let mut p = 0;
        for _ in 0..(c - 1) {
            x = x * 2 + p;
            p = (p + 1) % 2;
        }

        if x > n {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    } else {
        let mut x = 1;
        let mut p = 1;
        for _ in 0..(c - 1) {
            x = x * 2 + p;
            p = (p + 1) % 2;
        }

        if x > n {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }
}
